"""The classes of G7's allowed rows for P3 (F3, scenes), after the review of PR 261.

Run it on `allowed.json` beside it: `python3 classify_allowed.py`. It puts every row in one class
that a row of section 2.5 of docs/design/session-cycle.md covers, prints the count of each, and
fails when a row fits no class, or when a row says that something that exists today is gone, cut,
resized or narrowed outside the two classes that row 14 rules (a count, and Read leaving out the
plan section). It reads the reference beside it for nothing: each class is proved from the
difference itself and from the other rows of the same view.

The classes (the first five are the review's, with its checks):

  a  An unchanged item moves down by the scene rows' height: same name, x, width and height, and
     every such move in the view has the one same distance (row 14).
  b  New scene content on the Session page: a box, the words, a link, the second line, the
     menu (row 14).
  c  `Scene` as a line of the More menu (row 7).
  d  The reader's `Start fight` (rows 9 and 21).
  e  `Remove…` in a Ready to run row's menu (row 13).
  f  The fold: `N scenes` on the Tonight line (row 14, F3's fold).
  n  The heading's count: `TONIGHT 2` becomes `TONIGHT 3`, as row 14 says scenes are counted.
     Each gone count has its new count in the same view.
  r  Read leaves out the plan section that the scenes now show beside it (row 14): the
     section's heading and its task lines go from Read, the words below it move up, and the
     link that Read showed in a task line is now on the scene's row.
  t  An act's words: an act now shows or hides only words of the classes above.
"""
import collections
import json
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROWS = json.loads((HERE / "allowed.json").read_text())["rows"]

# The plain campaign's task lines, as its notes hold them.
SCENES = (
    "Read Emberfall Rumours",
    "Talk to Hobb the Miller about the ropes",
    "Ask about the toll",
    "Meet Hobb the Miller at the mill",
)
SCENE_PARTS = ("Talk to Hobb the Miller about the ", "ropes", "NPC · alive")
SCENE_LINKS = (
    "Hobb the Miller (opens Hobb the Miller)",
    "Emberfall Rumours (opens Emberfall Rumours)",
)
MENU = "\ue1fe"
COUNT = re.compile(r"^(TONIGHT|THIS SESSION) \d+$")
FOLD = re.compile(r"^\d+ scenes?$")
READ_PLAN = ("Plan for tonight", "Done", "Open") + tuple(" " + words for words in SCENES)
GLYPHS = ("\ue186", "\ue45e")


def bare(name):
    return re.sub(r"^(Button|CheckBox|Label|TextRun|Link): ", "", name)


def rect(text):
    return list(map(float, re.findall(r"-?\d+(?:\.\d+)?", text.split(", seen")[0])))


def scene_words(name):
    return (
        name in SCENES
        or name in SCENE_PARTS
        or name in SCENE_LINKS
        or any(name == prefix + words for prefix in ("More actions for ", "Done: ", "Open: ") for words in SCENES)
    )


def plan_view(screen):
    return screen.startswith("session-") or screen.startswith("lookup-reader ")


# The one distance that unchanged items move down in each view.
downs = collections.defaultdict(set)
for row in ROWS:
    d = row["difference"]
    if d["change"] == "moved" and "seen" not in d["detail"]:
        before, after = map(rect, d["detail"].split(" -> "))
        if before[0] == after[0] and before[2:] == after[2:] and after[1] > before[1]:
            downs[(d["screen"], d["view"])].add(round(after[1] - before[1], 1))
counts = collections.defaultdict(lambda: [0, 0])
for row in ROWS:
    d = row["difference"]
    if COUNT.match(bare(d["name"])):
        counts[(d["screen"], d["view"])][d["change"] == "new"] += 1

ACT_WORDS = set(SCENES) | set(SCENE_PARTS) | {"Scene", "", "Start fight", "Remove…", "Plan for tonight"}
ACT_WORDS |= {" " + words for words in SCENES}
ACT_WORDS |= set(GLYPHS) | {glyph + " " + words for glyph in GLYPHS for words in SCENES}


def act_word(word):
    return word in ACT_WORDS or COUNT.match(word) or FOLD.match(word)


def classify(d):
    name, screen, change, detail = bare(d["name"]), d["screen"], d["change"], d["detail"]
    if d["what"] == "act":
        words = re.findall(r"(?:appeared|went) [+-] ([^|]*?)(?: \||$)", detail)
        if words and all(act_word(word) for word in words):
            return "t"
        return None
    if "seen" in detail:
        return None
    if change == "moved":
        before, after = map(rect, detail.split(" -> "))
        same = before[0] == after[0] and before[2:] == after[2:]
        if same and after[1] > before[1] and len(downs[(screen, d["view"])]) == 1 and plan_view(screen):
            return "a"
        if same and after[1] < before[1] and screen.startswith("session-last") and before[0] >= 700:
            return "r"
        return None
    if COUNT.match(name) and change in ("new", "gone"):
        new, gone = counts[(screen, d["view"])]
        return "n" if new == gone else None
    if change == "gone":
        read_line = name in READ_PLAN or name in GLYPHS or any(
            name == glyph + " " + words for glyph in GLYPHS for words in SCENES
        )
        if screen.startswith("session-last") and read_line and rect(detail)[0] >= 700:
            return "r"
        return None
    if change == "resized":
        before, after = map(rect, detail.split(" -> "))
        if name in SCENE_LINKS and screen.startswith("session-last") and before[0] >= 700 > after[0]:
            return "r"
        return None
    if change != "new":
        return None
    if name == "Add Scene" or (screen == "session-tonight: More open" and name in ("Scene", "\ue186")):
        return "c"
    if name.startswith("Start a fight with ") or (
        (screen.startswith("lookup-reader ") or screen.startswith("world-reader")) and name in ("Start fight", "\ue3d0")
    ):
        return "d"
    if screen == "encounters: row menu" and name == "Remove…":
        return "e"
    if plan_view(screen) and FOLD.match(name):
        return "f"
    if plan_view(screen) and scene_words(name):
        return "b"
    if plan_view(screen) and name == MENU:
        # A painted glyph is matched by its order. A fight's menu glyph that moved down with its
        # row shows as new: it lies in a menu button that moved by the view's one distance.
        at = rect(detail)
        for other in ROWS:
            o = other["difference"]
            if (o["screen"], o["view"], o["change"]) == (screen, d["view"], "moved") and bare(
                o["name"]
            ).startswith("More actions for "):
                r = rect(o["detail"].split(" -> ")[1])
                if r[0] <= at[0] <= r[0] + r[2] and r[1] <= at[1] <= r[1] + r[3]:
                    return "a"
    if plan_view(screen) and name in (MENU, "\ue13a"):
        # A painted glyph is matched by its order: it must lie in a new scene menu's target.
        at = rect(detail)
        for other in ROWS:
            o = other["difference"]
            if (o["screen"], o["view"], o["change"]) == (screen, d["view"], "new") and any(
                bare(o["name"]) == "More actions for " + words for words in SCENES
            ):
                r = rect(o["detail"])
                if r[0] <= at[0] <= r[0] + r[2] and r[1] <= at[1] <= r[1] + r[3]:
                    return "b"
        return None
    return None


# Each class with the row of section 2.5 that covers it.
COVER = {"a": 14, "b": 14, "c": 7, "d": (9, 21), "e": 13, "f": 14, "n": 14, "r": 14, "t": (7, 9, 13, 14)}
found = collections.Counter()
outside = []
for index, row in enumerate(ROWS, 1):
    kind = classify(row["difference"])
    cover = COVER.get(kind)
    if kind is None or not (row["row"] == cover or (isinstance(cover, tuple) and row["row"] in cover)):
        outside.append((index, row["row"], kind, row["difference"]))
    found[kind] += 1
print(json.dumps({"total": len(ROWS), "classes": dict(sorted((k or "none", v) for k, v in found.items()))}, indent=1))
for index, design_row, kind, d in outside:
    print(f"outside: row {index} (section 2.5 row {design_row}, class {kind}): {d}", file=sys.stderr)
sys.exit(1 if outside else 0)
