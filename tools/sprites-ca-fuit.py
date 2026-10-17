#!/usr/bin/env python3
# Commit & Conquer — licence GPL-3.0-or-later.
#
# Dessine les sprites de « Ça fuit ! » (direction artistique « La plomberie
# de poche », docs/jeux/da/ca-fuit/) : les 34 cartes de tuyaux, vides et
# pleines d'eau, leur ombre, la fuite, la clôture et la mare, tracés au
# vecteur (cairo) aux couleurs et proportions relevées sur les planches de
# référence ; puis l'icône du jeu. Le reste du décor, le crapaud et les
# pictogrammes sont découpés dans les planches par tools/decoupe-ca-fuit.py.
#
#   tools/sprites-ca-fuit.py                 -> assets/sprites/ca-fuit/*.png
#   tools/sprites-ca-fuit.py --planche X.png  (en plus : une planche de contrôle)
#
# Une case = 256 px. Une carte debout occupe 2 × 1 cases ; son image fait
# 1,5 × 2,5 cases (une marge de 0,25 case tout autour, pour l'ombre portée),
# centrée sur la carte : le jeu l'affiche en `custom_size = (1.5, 2.5)`.
# Les sorties sont centrées sur les bords des cases, le tuyau s'arrête
# exactement au bord : deux cartes voisines se raccordent au pixel.
#
# Numérotation des sorties (fuite-core) sur une carte debout :
#         1
#      ┌─────┐
#    6 │     │ 2
#      │     │
#    5 │     │ 3
#      └─────┘
#         4
import math
import os
import random
import sys

import cairo

PX = 256  # pixels par case
MARGE = 0.25  # marge autour d'une carte, en cases
LARGEUR = 0.42  # largeur du tuyau, en cases
CONTOUR = 0.035  # épaisseur du contour bleu nuit
OMBRE = (0.07, 0.07)  # décalage de l'ombre portée (vers le bas à droite)
ALPHA_OMBRE = 0.28

# ---- Les couleurs, relevées sur les planches de référence ---------------------
def rgb(h):
    h = h.lstrip("#")
    return tuple(int(h[i : i + 2], 16) / 255 for i in (0, 2, 4))

NUIT = rgb("#0E1B3A")  # contours
CUIVRE = dict(brillant=rgb("#FFE9B8"), clair=rgb("#F99657"), moyen=rgb("#F48B4A"), fonce=rgb("#D46F3C"), ombre=rgb("#9A5233"))
GRIS = dict(brillant=rgb("#F2F2F0"), clair=rgb("#C4C4C2"), moyen=rgb("#9A9A9A"), fonce=rgb("#6E7277"), ombre=rgb("#4A4F5A"))
EAU = dict(ombre=rgb("#1788D0"), fonce=rgb("#1788D0"), moyen=rgb("#00B4FD"), clair=rgb("#62DAFF"), brillant=rgb("#DDF8FF"))
ROUGE = dict(ombre=rgb("#B61635"), moyen=rgb("#E7233C"), clair=rgb("#F4505E"), brillant=rgb("#FFD7DC"))
IVOIRE = rgb("#FDF2DF")
VERT_TOUFFE = (rgb("#5FBA2E"), rgb("#4CA82A"), rgb("#7CCB38"))
VERT_BUISSON = (rgb("#12884E"), rgb("#1FA05A"), rgb("#39A445"))
BOIS = dict(clair=rgb("#F9AF64"), moyen=rgb("#E59C59"), fonce=rgb("#B9743A"))
CAILLOU = dict(clair=rgb("#D9D2CA"), moyen=rgb("#B5A99F"), fonce=rgb("#8F8479"))
ROSEAU = rgb("#9F5729")
OR = dict(fonce=rgb("#C98A12"), moyen=rgb("#F5B82E"), clair=rgb("#FFD966"), brillant=rgb("#FFF2B8"))

# Directions en coordonnées image (y vers le bas).
N, E, S, W = (0, -1), (1, 0), (0, 1), (-1, 0)


# ---- Les formes (mêmes règles que fuite-core) ---------------------------------
def demi_tour(bits):
    return ((bits & 0b111) << 3) | (bits >> 3)

def sorties(bits):
    return [s for s in range(1, 7) if bits & (1 << (s - 1))]

def canonique(bits):
    autre = demi_tour(bits)
    return autre if sorties(autre) < sorties(bits) else bits

def toutes_les_formes():
    formes = sorted({canonique(b) for b in range(1, 0b111111)}, key=lambda b: (len(sorties(b)), sorties(b)))
    assert len(formes) == 34, len(formes)
    return formes

def nom(bits):
    return "-".join(str(s) for s in sorties(bits))

FUITE = 0b111111


# ---- La géométrie d'une carte -------------------------------------------------
# Chaque case a un centre et un ensemble de directions reliées : ses sorties,
# plus la direction de l'autre case (l'« épine » qui relie les deux cases).
def cases(bits):
    haut = {S}
    bas = {N}
    if bits & 0b000001: haut.add(N)  # 1
    if bits & 0b000010: haut.add(E)  # 2
    if bits & 0b100000: haut.add(W)  # 6
    if bits & 0b000100: bas.add(E)   # 3
    if bits & 0b001000: bas.add(S)   # 4
    if bits & 0b010000: bas.add(W)   # 5
    return [((0.5, 0.5), haut, S), ((0.5, 1.5), bas, N)]


def ajouter(p, d, k):
    return (p[0] + d[0] * k, p[1] + d[1] * k)


def chemins(bits, debordement=0.3, recouvrement=0.06, centre_plus=0.0):
    """Les tracés du tuyau (listes de segments et d'arcs), par case.

    Les bouts qui sortent de la carte sont prolongés de `debordement` (on
    rogne ensuite au bord) ; l'épine est prolongée de `recouvrement` dans
    l'autre case pour qu'il n'y ait pas de couture. Renvoie aussi les
    extrémités bouchées : (point du centre, direction du bout mort)."""
    traces = []
    bouchons = []
    for centre, dirs, epine in cases(bits):
        def prolongement(d):
            return recouvrement if d == epine else debordement
        if len(dirs) == 1:
            # Un bout mort : l'épine arrive au centre, un bouchon la ferme.
            d = epine
            a = ajouter(centre, d, 0.5 + prolongement(d))
            b = ajouter(centre, d, -0.08)
            traces.append([("ligne", a, b)])
            bouchons.append((centre, (-d[0], -d[1])))
        elif len(dirs) == 2 and sum(d[0] for d in dirs) == 0 and sum(d[1] for d in dirs) == 0:
            d1, d2 = sorted(dirs)
            traces.append([("ligne", ajouter(centre, d1, 0.5 + prolongement(d1)), ajouter(centre, d2, 0.5 + prolongement(d2)))])
        elif len(dirs) == 2:
            d1, d2 = sorted(dirs)
            coin = ajouter(ajouter(centre, d1, 0.5), d2, 0.5)
            a = ajouter(centre, d1, 0.5)
            b = ajouter(centre, d2, 0.5)
            traces.append([
                ("ligne", ajouter(a, d1, prolongement(d1)), a),
                ("arc", coin, 0.5, a, b),
                ("ligne", b, ajouter(b, d2, prolongement(d2))),
            ])
        else:
            # Une jonction : les branches qui se font face forment le tuyau
            # traversant, tracé en dernier (par-dessus les autres) pour que
            # ses bandes de lumière cachent le croisement.
            def traversant(d):
                return (-d[0], -d[1]) in dirs
            for d in sorted(dirs, key=lambda d: (traversant(d), d[1] != 0)):
                traces.append([("ligne", ajouter(centre, d, -(LARGEUR / 2 + CONTOUR + centre_plus)), ajouter(centre, d, 0.5 + prolongement(d)))])
    return traces, bouchons


def angle(p, c):
    return math.atan2(p[1] - c[1], p[0] - c[0])


def tracer(ctx, traces, dx=0.0, dy=0.0):
    """Ajoute les tracés au chemin courant, décalés de (dx, dy)."""
    for trace in traces:
        premier = True
        for seg in trace:
            if seg[0] == "ligne":
                _, a, b = seg
                if premier:
                    ctx.move_to(a[0] + dx, a[1] + dy)
                    premier = False
                ctx.line_to(b[0] + dx, b[1] + dy)
            else:
                _, c, r, a, b = seg
                a0, a1 = angle(a, c), angle(b, c)
                if premier:
                    ctx.move_to(a[0] + dx, a[1] + dy)
                    premier = False
                if (a1 - a0) % (2 * math.pi) < math.pi:
                    ctx.arc(c[0] + dx, c[1] + dy, r, a0, a1)
                else:
                    ctx.arc_negative(c[0] + dx, c[1] + dy, r, a0, a1)


def tube(ctx, traces, largeur, couleurs, bandes, clip=None, reflets=False):
    """Un tube le long des tracés : des bandes de couleur décalées vers la
    lumière (en haut à gauche), rognées à la forme du tube. `bandes` = liste
    de (largeur relative, décalage relatif vers la lumière, couleur, alpha).
    Les tracés sont peints l'un après l'autre : le dernier recouvre les
    autres là où ils se croisent."""
    ctx.save()
    if clip:
        ctx.rectangle(*clip)
        ctx.clip()
    ctx.set_line_cap(cairo.LINE_CAP_BUTT)
    ctx.set_line_join(cairo.LINE_JOIN_ROUND)
    for trace in traces:
        ctx.push_group()
        ctx.set_source_rgb(*couleurs["ombre"])
        ctx.paint()
        for (rel, dec, couleur, alpha) in bandes:
            ctx.new_path()
            tracer(ctx, [trace], -dec * largeur, -dec * largeur)
            ctx.set_line_width(rel * largeur)
            ctx.set_source_rgba(*couleur, alpha)
            ctx.stroke()
        motif = ctx.pop_group()
        ctx.new_path()
        tracer(ctx, [trace])
        ctx.set_line_width(largeur)
        ctx.set_source(motif)
        ctx.stroke()
        if reflets:
            # Des reflets : des pastilles blanches en pointillés, décalées
            # vers la lumière (la branche suivante les recouvre au croisement).
            ctx.save()
            ctx.new_path()
            tracer(ctx, [trace], -largeur * 0.2, -largeur * 0.2)
            ctx.set_line_width(largeur * 0.16)
            ctx.set_line_cap(cairo.LINE_CAP_ROUND)
            ctx.set_dash([0.10, 0.30], 0.12)
            ctx.set_source_rgba(*couleurs["brillant"], 0.95)
            ctx.stroke()
            ctx.restore()
    ctx.restore()


BANDES_CUIVRE = [(0.90, 0.05, CUIVRE["fonce"], 1), (0.64, 0.18, CUIVRE["moyen"], 1), (0.36, 0.32, CUIVRE["clair"], 1), (0.09, 0.355, CUIVRE["brillant"], 0.85)]
BANDES_EAU = [(0.90, 0.05, EAU["moyen"], 1), (0.42, 0.24, EAU["clair"], 1)]
BANDES_GRIS = [(0.90, 0.05, GRIS["moyen"], 1), (0.60, 0.20, GRIS["clair"], 1), (0.14, 0.34, GRIS["brillant"], 0.9)]


def contour(ctx, traces, largeur, clip=None):
    ctx.save()
    if clip:
        ctx.rectangle(*clip)
        ctx.clip()
    ctx.new_path()
    tracer(ctx, traces)
    ctx.set_line_cap(cairo.LINE_CAP_BUTT)
    ctx.set_line_join(cairo.LINE_JOIN_ROUND)
    ctx.set_line_width(largeur)
    ctx.set_source_rgb(*NUIT)
    ctx.stroke()
    ctx.restore()


def rect_arrondi(ctx, x, y, w, h, r):
    ctx.new_sub_path()
    ctx.arc(x + w - r, y + r, r, -math.pi / 2, 0)
    ctx.arc(x + w - r, y + h - r, r, 0, math.pi / 2)
    ctx.arc(x + r, y + h - r, r, math.pi / 2, math.pi)
    ctx.arc(x + r, y + r, r, math.pi, 3 * math.pi / 2)
    ctx.close_path()


def collier(ctx, centre, d, epaisseur=0.11, depassement=0.035, rivets=True, seulement_forme=False):
    """Un collier gris riveté, en travers du tuyau en `centre`, le tuyau
    allant dans la direction `d`."""
    ctx.save()
    ctx.translate(*centre)
    ctx.rotate(math.atan2(d[1], d[0]))
    w = LARGEUR + 2 * depassement
    # Repère local : x le long du tuyau, y en travers.
    rect_arrondi(ctx, -epaisseur / 2, -w / 2, epaisseur, w, 0.02)
    if seulement_forme:
        ctx.restore()
        return
    ctx.set_source_rgb(*NUIT)
    ctx.set_line_width(2 * CONTOUR)
    ctx.stroke_preserve()
    # Dégradé en travers : la lumière vient du haut à gauche. En repère
    # local on ne sait pas où est le haut : on prend un dégradé symétrique
    # clair au milieu, un peu plus clair du côté de la lumière.
    cote = -1 if (d[1] < 0 or d[0] > 0) else 1
    g = cairo.LinearGradient(0, -w / 2, 0, w / 2)
    if cote > 0:
        g.add_color_stop_rgb(0.0, *GRIS["fonce"]); g.add_color_stop_rgb(0.25, *GRIS["moyen"]); g.add_color_stop_rgb(0.62, *GRIS["clair"]); g.add_color_stop_rgb(0.86, *GRIS["brillant"]); g.add_color_stop_rgb(1.0, *GRIS["clair"])
    else:
        g.add_color_stop_rgb(0.0, *GRIS["clair"]); g.add_color_stop_rgb(0.14, *GRIS["brillant"]); g.add_color_stop_rgb(0.38, *GRIS["clair"]); g.add_color_stop_rgb(0.75, *GRIS["moyen"]); g.add_color_stop_rgb(1.0, *GRIS["fonce"])
    ctx.set_source(g)
    ctx.fill()
    if rivets:
        for ry in (-w * 0.28, w * 0.28):
            ctx.arc(0, ry, 0.022, 0, 2 * math.pi)
            ctx.set_source_rgb(*GRIS["ombre"])
            ctx.fill()
            ctx.arc(-0.005, ry - 0.005, 0.011, 0, 2 * math.pi)
            ctx.set_source_rgb(*GRIS["brillant"])
            ctx.fill()
    ctx.restore()


def bouchon(ctx, centre, d, seulement_forme=False):
    """Le bouchon rouge qui ferme un bout mort, en `centre`, pointant vers `d`
    (vers l'extérieur du tuyau) : un capuchon plat, une bride un peu plus
    large que le tuyau, aux coins arrondis, avec une bande claire."""
    ctx.save()
    ctx.translate(*centre)
    ctx.rotate(math.atan2(d[1], d[0]))
    w = LARGEUR + 0.10
    ep = 0.17
    # Repère local : x le long du tuyau (vers le bout), y en travers.
    rect_arrondi(ctx, -0.02, -w / 2, ep, w, 0.045)
    if seulement_forme:
        ctx.restore()
        return
    ctx.set_source_rgb(*NUIT)
    ctx.set_line_width(2 * CONTOUR)
    ctx.stroke_preserve()
    g = cairo.LinearGradient(0, -w / 2, 0, w / 2)
    cote = -1 if (d[1] < 0 or d[0] > 0) else 1
    if cote > 0:
        g.add_color_stop_rgb(0.0, *ROUGE["ombre"]); g.add_color_stop_rgb(0.45, *ROUGE["moyen"]); g.add_color_stop_rgb(0.8, *ROUGE["clair"]); g.add_color_stop_rgb(1.0, *ROUGE["moyen"])
    else:
        g.add_color_stop_rgb(0.0, *ROUGE["moyen"]); g.add_color_stop_rgb(0.2, *ROUGE["clair"]); g.add_color_stop_rgb(0.55, *ROUGE["moyen"]); g.add_color_stop_rgb(1.0, *ROUGE["ombre"])
    ctx.set_source(g)
    ctx.fill()
    # La bande claire sur l'épaisseur, côté tuyau.
    rect_arrondi(ctx, 0.0, -w / 2 + 0.03, 0.035, w - 0.06, 0.015)
    ctx.set_source_rgba(*ROUGE["brillant"], 0.8)
    ctx.fill()
    ctx.restore()
    # Le collier juste avant le capuchon.
    collier(ctx, ajouter(centre, d, -0.10), d, epaisseur=0.10)


def surface_carte():
    s = cairo.ImageSurface(cairo.FORMAT_ARGB32, int((1 + 2 * MARGE) * PX), int((2 + 2 * MARGE) * PX))
    ctx = cairo.Context(s)
    ctx.scale(PX, PX)
    ctx.translate(MARGE, MARGE)
    ctx.set_antialias(cairo.ANTIALIAS_BEST)
    return s, ctx


RECT_CARTE = (0, 0, 1, 2)


def dessiner_carte(bits, eau=False, fissure=False):
    s, ctx = surface_carte()
    traces, bouchons = chemins(bits)
    contour(ctx, traces, LARGEUR + 2 * CONTOUR, clip=RECT_CARTE)
    tube(ctx, traces, LARGEUR, CUIVRE, BANDES_CUIVRE, clip=RECT_CARTE)
    if eau:
        # Le canal : l'eau entre les deux parois de cuivre.
        canal = LARGEUR * 0.64
        contour_eau = [(1.0, 0.0, EAU["fonce"], 1)]
        tube(ctx, traces, canal + 0.03, EAU, contour_eau, clip=RECT_CARTE)
        tube(ctx, traces, canal, EAU, BANDES_EAU, clip=RECT_CARTE, reflets=True)
    # Les colliers aux sorties, un peu en retrait du bord.
    for (case, direction) in sorties_geometrie(bits):
        collier(ctx, ajouter(case, direction, 0.5 - 0.14), direction)
    for (centre, d) in bouchons:
        bouchon(ctx, centre, d)
    if fissure:
        dessiner_fissure(ctx)
    return s


def sorties_geometrie(bits):
    """(centre de la case, direction) pour chaque sortie ouverte."""
    out = []
    for centre, dirs, epine in cases(bits):
        for d in dirs:
            if d != epine:
                out.append((centre, d))
    return out


def dessiner_ombre(bits):
    """L'ombre portée de la carte : sa silhouette, décalée vers le bas à
    droite, rognée au bord droit et bas de la carte seulement (ce qui déborde
    à droite et en bas s'affiche sous la carte voisine)."""
    s, ctx = surface_carte()
    traces, bouchons = chemins(bits)
    ctx.save()
    ctx.rectangle(0, 0, 1 + OMBRE[0], 2 + OMBRE[1])
    ctx.clip()
    ctx.translate(*OMBRE)
    ctx.push_group()
    ctx.new_path()
    tracer(ctx, traces)
    ctx.set_line_cap(cairo.LINE_CAP_BUTT)
    ctx.set_line_join(cairo.LINE_JOIN_ROUND)
    ctx.set_line_width(LARGEUR + 2 * CONTOUR)
    ctx.set_source_rgb(*NUIT)
    ctx.save()
    ctx.rectangle(*RECT_CARTE)
    ctx.clip()
    ctx.stroke()
    ctx.restore()
    for (case, direction) in sorties_geometrie(bits):
        collier(ctx, ajouter(case, direction, 0.5 - 0.14), direction, seulement_forme=True)
        ctx.set_source_rgb(*NUIT)
        ctx.set_line_width(2 * CONTOUR)
        ctx.stroke_preserve()
        ctx.fill()
    for (centre, d) in bouchons:
        bouchon(ctx, centre, d, seulement_forme=True)
        ctx.set_source_rgb(*NUIT)
        ctx.set_line_width(2 * CONTOUR)
        ctx.stroke_preserve()
        ctx.fill()
        collier(ctx, ajouter(centre, d, -0.10), d, epaisseur=0.10, seulement_forme=True)
        ctx.stroke_preserve()
        ctx.fill()
    ctx.pop_group_to_source()
    ctx.paint_with_alpha(ALPHA_OMBRE)
    ctx.restore()
    return s


def dessiner_fissure(ctx):
    """La fente de la fuite, au milieu de la carte, en travers de l'épine."""
    rnd = random.Random(3)
    cx, cy = 0.5, 1.0
    # Une fente en zigzag, verticale (le long de l'épine), avec des branches.
    pts = [(cx + rnd.uniform(-0.05, 0.05), cy - 0.34 + i * 0.68 / 8) for i in range(9)]
    pts[0] = (cx - 0.02, cy - 0.36)
    pts[-1] = (cx + 0.03, cy + 0.33)
    ctx.save()
    ctx.set_line_cap(cairo.LINE_CAP_ROUND)
    ctx.set_line_join(cairo.LINE_JOIN_ROUND)
    def chemin(epaisseur):
        ctx.new_path()
        ctx.move_to(*pts[0])
        for p in pts[1:]:
            ctx.line_to(*p)
        ctx.set_line_width(epaisseur)
        for (i, dx, dy) in [(2, -0.14, -0.05), (5, 0.14, 0.03), (6, -0.11, 0.08)]:
            ctx.move_to(*pts[i])
            ctx.line_to(pts[i][0] + dx, pts[i][1] + dy)
    chemin(0.065)
    ctx.set_source_rgb(*NUIT)
    ctx.stroke()
    chemin(0.028)
    ctx.set_source_rgb(*EAU["clair"])
    ctx.stroke()
    ctx.restore()


# ---- Le décor -------------------------------------------------------------------
def surface(cases_l, cases_h):
    s = cairo.ImageSurface(cairo.FORMAT_ARGB32, int(cases_l * PX), int(cases_h * PX))
    ctx = cairo.Context(s)
    ctx.scale(PX, PX)
    ctx.set_antialias(cairo.ANTIALIAS_BEST)
    return s, ctx


def poteau(ctx, x, y, h=0.5, w=0.16):
    rect_arrondi(ctx, x - w / 2, y - h, w, h, 0.03)
    ctx.set_source_rgb(*NUIT)
    ctx.set_line_width(CONTOUR * 1.4)
    ctx.stroke_preserve()
    g = cairo.LinearGradient(x - w / 2, 0, x + w / 2, 0)
    g.add_color_stop_rgb(0, *BOIS["clair"])
    g.add_color_stop_rgb(0.55, *BOIS["moyen"])
    g.add_color_stop_rgb(1, *BOIS["fonce"])
    ctx.set_source(g)
    ctx.fill()
    # Le dessus du poteau.
    rect_arrondi(ctx, x - w / 2, y - h, w, 0.07, 0.03)
    ctx.set_source_rgb(*BOIS["clair"])
    ctx.fill()


def dessiner_cloture():
    """Un tronçon de clôture horizontal sur 2 × 1 cases : deux poteaux (aux
    quarts) et une lisse. Mis bout à bout, les lisses se raccordent."""
    s, ctx = surface(2, 1)
    y_sol = 0.78
    # La lisse, d'un bord à l'autre.
    rect_arrondi(ctx, -0.1, y_sol - 0.34, 2.2, 0.14, 0.02)
    ctx.set_source_rgb(*NUIT)
    ctx.set_line_width(CONTOUR * 1.4)
    ctx.stroke_preserve()
    g = cairo.LinearGradient(0, y_sol - 0.34, 0, y_sol - 0.20)
    g.add_color_stop_rgb(0, *BOIS["clair"])
    g.add_color_stop_rgb(0.5, *BOIS["moyen"])
    g.add_color_stop_rgb(1, *BOIS["fonce"])
    ctx.set_source(g)
    ctx.fill()
    for x in (0.5, 1.5):
        poteau(ctx, x, y_sol)
    return s


def blob(ctx, cx, cy, rayon, rnd, n=9, irregularite=0.3):
    """Une forme molle fermée (courbes de Bézier entre points d'un cercle bruité)."""
    pts = []
    for i in range(n):
        a = i * 2 * math.pi / n
        r = rayon * (1 + rnd.uniform(-irregularite, irregularite))
        pts.append((cx + math.cos(a) * r, cy + math.sin(a) * r * 0.82))
    ctx.new_sub_path()
    def mid(a, b):
        return ((a[0] + b[0]) / 2, (a[1] + b[1]) / 2)
    ctx.move_to(*mid(pts[-1], pts[0]))
    for i in range(n):
        p = pts[i]
        m = mid(p, pts[(i + 1) % n])
        ctx.curve_to(p[0], p[1], p[0], p[1], m[0], m[1])
    ctx.close_path()


def dessiner_mare():
    """La mare : 3 × 3 cases, bords mous, deux bleus, nénuphars et roseaux."""
    s, ctx = surface(3, 3)
    rnd = random.Random(21)
    blob(ctx, 1.5, 1.55, 1.15, rnd, n=11, irregularite=0.18)
    ctx.set_source_rgb(*NUIT)
    ctx.set_line_width(CONTOUR * 1.6)
    ctx.stroke_preserve()
    ctx.set_source_rgb(*EAU["fonce"])
    ctx.fill()
    blob(ctx, 1.45, 1.5, 0.85, rnd, n=9, irregularite=0.2)
    ctx.set_source_rgb(*EAU["moyen"])
    ctx.fill()
    for _ in range(4):
        ctx.save()
        ctx.translate(1.0 + rnd.uniform(0, 1.0), 1.0 + rnd.uniform(0, 1.0))
        ctx.rotate(rnd.uniform(-0.4, 0.4))
        ctx.scale(0.16, 0.05)
        ctx.arc(0, 0, 1, 0, 2 * math.pi)
        ctx.restore()
        ctx.set_source_rgba(*EAU["brillant"], 0.85)
        ctx.fill()
    return s


# ---- L'icône du jeu ----------------------------------------------------------------
def dessiner_icone(eclaboussure, taille=256):
    """L'icône : la fuite qui gicle, sur l'herbe, dans un carré arrondi."""
    s = cairo.ImageSurface(cairo.FORMAT_ARGB32, taille, taille)
    ctx = cairo.Context(s)
    ctx.scale(taille, taille)
    ctx.set_antialias(cairo.ANTIALIAS_BEST)
    rect_arrondi(ctx, 0.02, 0.02, 0.96, 0.96, 0.2)
    ctx.set_source_rgb(*rgb("#A3D46B"))
    ctx.fill_preserve()
    ctx.set_source_rgb(*NUIT)
    ctx.set_line_width(0.035)
    ctx.stroke()
    # Damier doux.
    ctx.save()
    rect_arrondi(ctx, 0.02, 0.02, 0.96, 0.96, 0.2)
    ctx.clip()
    for i in range(4):
        for j in range(4):
            if (i + j) % 2:
                ctx.rectangle(i * 0.25, j * 0.25, 0.25, 0.25)
    ctx.set_source_rgba(*rgb("#8FC95A"), 0.6)
    ctx.fill()
    ctx.restore()
    # Un tuyau horizontal fendu, plein d'eau.
    carte = dessiner_carte(0b001001, eau=True, fissure=True)  # 1-4 : le droit
    ctx.save()
    ctx.translate(0.5, 0.56)
    ctx.rotate(-math.pi / 2)
    ech = 0.42
    ctx.scale(ech, ech)
    ctx.translate(-(1 + 2 * MARGE) / 2, -(2 + 2 * MARGE) / 2)
    ctx.scale(1 / PX, 1 / PX)
    ctx.set_source_surface(dessiner_ombre(0b001001), 0, 0)
    ctx.paint()
    ctx.set_source_surface(carte, 0, 0)
    ctx.paint()
    ctx.restore()
    ctx.save()
    jet = cairo.ImageSurface.create_from_png(eclaboussure)
    ctx.translate(0.5, 0.30)
    ech = 0.5 / jet.get_width()
    ctx.scale(ech, ech)
    ctx.translate(-jet.get_width() / 2, -jet.get_height() / 2)
    ctx.set_source_surface(jet, 0, 0)
    ctx.paint()
    ctx.restore()
    return s


# ---- Fabrication ----------------------------------------------------------------------
def main():
    racine = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    sortie = os.path.join(racine, "assets", "sprites", "ca-fuit")
    os.makedirs(sortie, exist_ok=True)
    for f in os.listdir(sortie):
        os.remove(os.path.join(sortie, f))
    images = {}
    for bits in toutes_les_formes():
        n = nom(bits)
        images[f"carte-{n}"] = dessiner_carte(bits)
        images[f"carte-{n}-eau"] = dessiner_carte(bits, eau=True)
        images[f"carte-{n}-ombre"] = dessiner_ombre(bits)
    images["fuite"] = dessiner_carte(FUITE, eau=True, fissure=True)
    images["fuite-ombre"] = dessiner_ombre(FUITE)
    images["cloture"] = dessiner_cloture()
    images["mare"] = dessiner_mare()
    for n, s in images.items():
        s.write_to_png(os.path.join(sortie, f"{n}.png"))
    print(f"{len(images)} sprites dans {sortie}")
    # Le décor, le crapaud et les pictogrammes viennent des planches dessinées.
    import importlib.util
    spec = importlib.util.spec_from_file_location("decoupe", os.path.join(racine, "tools", "decoupe-ca-fuit.py"))
    decoupe = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(decoupe)
    decoupe.decouper()
    icone = dessiner_icone(os.path.join(sortie, "eclaboussure.png"))
    for chemin in (os.path.join(racine, "packaging", "icons", "ca-fuit.png"), os.path.join(racine, "site", "static", "img", "jeux", "ca-fuit.png")):
        icone.write_to_png(chemin)
        print(chemin)
    if "--planche" in sys.argv:
        planche(images, sys.argv[sys.argv.index("--planche") + 1])


def planche(images, chemin):
    """Une planche de contrôle : les cartes (ombre + carte, puis pleine) et le reste, sur fond d'herbe."""
    from PIL import Image
    fond = Image.new("RGBA", (10 * 200, 8 * 360 + 700), (163, 212, 107, 255))
    def coller(nom_image, x, y, ech):
        s = images[nom_image]
        im = Image.frombuffer("RGBA", (s.get_width(), s.get_height()), bytes(s.get_data()), "raw", "BGRa", s.get_stride())
        im = im.resize((max(1, int(im.width * ech)), max(1, int(im.height * ech))), Image.LANCZOS)
        fond.alpha_composite(im, (x, y))
    i = 0
    for bits in toutes_les_formes():
        n = nom(bits)
        x, y = (i % 10) * 200 + 10, (i // 10) * 360 + 10
        coller(f"carte-{n}-ombre", x, y, 0.5)
        coller(f"carte-{n}", x, y, 0.5)
        i += 1
    i = 0
    for bits in toutes_les_formes():
        n = nom(bits)
        x, y = (i % 10) * 200 + 10, 4 * 360 + (i // 10) * 360 + 10
        coller(f"carte-{n}-ombre", x, y, 0.5)
        coller(f"carte-{n}-eau", x, y, 0.5)
        i += 1
    y0 = 8 * 360 + 10
    coller("fuite-ombre", 10, y0, 0.5); coller("fuite", 10, y0, 0.5)
    x = 260
    for n in []:
        coller(n, x, y0, 0.5)
        x += images[n].get_width() // 2 + 10
        if x > 1800:
            x = 260; y0 += 200
    y0 += 220
    coller("cloture", 460, y0, 0.5); coller("cloture", 460 + 256, y0, 0.5); coller("mare", 1000, y0, 0.5)
    fond.save(chemin)
    print(chemin)


if __name__ == "__main__":
    main()
