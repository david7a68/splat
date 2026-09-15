---
splat_schema: 1
splat_id: 6c344711-b7dc-5066-ba78-2d8444c7ba8f
splat_kind: rule
splat_key: dnd2024:rules/trinkets
splat_name: Trinkets
splat_origin:
  key: dnd2024:reference/pages/026
splat_sources:
- title: SRD 5.2.1
  locator: pp. 26, 27
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/trinkets
  parameters:
    creation_roll_optional: true
    rolls_per_character_creation: 1
    roll:
      dice:
        count: 1
        sides: 100
    item_size: tiny
    results:
    - roll: 1
      appearance: A mummified goblin hand
    - roll: 2
      appearance: A crystal that faintly glows in moonlight
      properties:
        glow: faint
        trigger: moonlight
    - roll: 3
      appearance: A gold coin minted in an unknown land
    - roll: 4
      appearance: A diary written in a language you don’t know
      properties:
        language_known_to_owner: false
    - roll: 5
      appearance: A brass ring that never tarnishes
      properties:
        tarnishes: false
    - roll: 6
      appearance: An old chess piece made from glass
    - roll: 7
      appearance: A pair of knucklebone dice, each with a skull symbol on the side that would normally
        show six pips
      properties:
        count: 2
        die_six_face: skull
    - roll: 8
      appearance: A small idol depicting a nightmarish creature that gives you unsettling dreams when
        you sleep near it
      properties:
        sleep_nearby_dreams: unsettling
    - roll: 9
      appearance: A lock of someone’s hair
    - roll: 10
      appearance: The deed for a parcel of land in a realm unknown to you
    - roll: 11
      appearance: A 1-ounce block made from an unknown material
      properties:
        weight_ounces: 1
    - roll: 12
      appearance: A small cloth doll skewered with needles
    - roll: 13
      appearance: A tooth from an unknown beast
    - roll: 14
      appearance: An enormous scale, perhaps from a dragon
    - roll: 15
      appearance: A bright-green feather
    - roll: 16
      appearance: An old divination card bearing your likeness
    - roll: 17
      appearance: A glass orb filled with moving smoke
    - roll: 18
      appearance: A 1-pound egg with a bright-red shell
      properties:
        weight_pounds: 1
    - roll: 19
      appearance: A pipe that blows bubbles
      properties:
        emission: bubbles
    - roll: 20
      appearance: A glass jar containing a bit of flesh floating in pickling fluid
    - roll: 21
      appearance: A gnome-crafted music box that plays a song you dimly remember from your childhood
      properties:
        plays_music: true
    - roll: 22
      appearance: A wooden statuette of a smug halfling
    - roll: 23
      appearance: A brass orb etched with strange runes
    - roll: 24
      appearance: A multicolored stone disk
    - roll: 25
      appearance: A silver icon of a raven
    - roll: 26
      appearance: A bag containing forty-seven teeth, one of which is rotten
      properties:
        teeth_count: 47
        rotten_teeth: 1
    - roll: 27
      appearance: A shard of obsidian that always feels warm to the touch
      properties:
        touch_temperature: warm
    - roll: 28
      appearance: A dragon’s talon strung on a leather necklace
    - roll: 29
      appearance: A pair of old socks
      properties:
        count: 2
    - roll: 30
      appearance: A blank book whose pages refuse to hold ink, chalk, graphite, or any other marking
      properties:
        marking_retention: false
        media:
        - ink
        - chalk
        - graphite
        - other
    - roll: 31
      appearance: A silver badge that is a five-pointed star
      properties:
        star_points: 5
    - roll: 32
      appearance: A knife that belonged to a relative
    - roll: 33
      appearance: A glass vial filled with nail clippings
    - roll: 34
      appearance: A rectangular metal device with two tiny metal cups on one end that throws sparks when
        wet
      properties:
        metal_cups: 2
        wet_emits_sparks: true
    - roll: 35
      appearance: A white, sequined glove sized for a human
    - roll: 36
      appearance: A vest with one hundred tiny pockets
      properties:
        pockets: 100
    - roll: 37
      appearance: A weightless stone
      properties:
        weight_pounds: 0
    - roll: 38
      appearance: A sketch of a goblin
    - roll: 39
      appearance: An empty glass vial that smells of perfume
    - roll: 40
      appearance: A gemstone that looks like a lump of coal when examined by anyone but you
      properties:
        owner_appearance: gemstone
        others_appearance: coal
    - roll: 41
      appearance: A scrap of cloth from an old banner
    - roll: 42
      appearance: A rank insignia from a lost legionnaire
    - roll: 43
      appearance: A silver bell without a clapper
      properties:
        clapper: false
    - roll: 44
      appearance: A mechanical canary inside a lamp
    - roll: 45
      appearance: A miniature chest carved to look like it has numerous feet on the bottom
    - roll: 46
      appearance: A dead sprite inside a clear glass bottle
    - roll: 47
      appearance: A metal can that has no opening but sounds as if it is filled with liquid, sand, spiders,
        or broken glass (your choice)
      properties:
        opening: false
        sound_choice:
        - liquid
        - sand
        - spiders
        - broken_glass
    - roll: 48
      appearance: A glass orb filled with water, in which swims a clockwork goldfish
    - roll: 49
      appearance: A silver spoon with an M engraved on the handle
    - roll: 50
      appearance: A whistle made from gold-colored wood
    - roll: 51
      appearance: A dead scarab beetle the size of your hand
    - roll: 52
      appearance: Two toy soldiers, one missing a head
      properties:
        soldiers: 2
        missing_heads: 1
    - roll: 53
      appearance: A small box filled with different-sized buttons
    - roll: 54
      appearance: A candle that can’t be lit
      properties:
        can_be_lit: false
    - roll: 55
      appearance: A miniature cage with no door
      properties:
        door: false
    - roll: 56
      appearance: An old key
    - roll: 57
      appearance: An indecipherable treasure map
    - roll: 58
      appearance: A hilt from a broken sword
    - roll: 59
      appearance: A rabbit’s foot
    - roll: 60
      appearance: A glass eye
    - roll: 61
      appearance: A cameo of a hideous person
    - roll: 62
      appearance: A silver skull the size of a coin
    - roll: 63
      appearance: An alabaster mask
    - roll: 64
      appearance: A cone of sticky black incense that stinks
    - roll: 65
      appearance: A nightcap that gives you pleasant dreams when you wear it
      properties:
        while_worn_dreams: pleasant
    - roll: 66
      appearance: A single caltrop made from bone
    - roll: 67
      appearance: A gold monocle frame without the lens
      properties:
        lens: false
    - roll: 68
      appearance: A 1-inch cube, each side a different color
      properties:
        cube_edge_inches: 1
        side_colors_distinct: true
    - roll: 69
      appearance: A crystal doorknob
    - roll: 70
      appearance: A packet filled with pink dust
    - roll: 71
      appearance: A fragment of a beautiful song, written as musical notes on two pieces of parchment
      properties:
        parchment_pieces: 2
    - roll: 72
      appearance: A silver teardrop earring containing a real teardrop
    - roll: 73
      appearance: An eggshell painted with scenes of misery in disturbing detail
    - roll: 74
      appearance: A fan that, when unfolded, shows a sleepy cat
    - roll: 75
      appearance: A set of bone pipes
    - roll: 76
      appearance: A four-leaf clover pressed inside a book discussing manners and etiquette
      properties:
        clover_leaves: 4
    - roll: 77
      appearance: A sheet of parchment upon which is drawn a mechanical contraption
    - roll: 78
      appearance: An ornate scabbard that fits no blade you have found
    - roll: 79
      appearance: An invitation to a party where a murder happened
    - roll: 80
      appearance: A bronze pentacle with an etching of a rat’s head in its center
    - roll: 81
      appearance: A purple handkerchief embroidered with the name of an archmage
    - roll: 82
      appearance: Half a floor plan for a temple, a castle, or another structure
      properties:
        plan_fraction: 0.5
        structure_choice:
        - temple
        - castle
        - other_structure
    - roll: 83
      appearance: A bit of folded cloth that, when unfolded, turns into a stylish cap
      properties:
        unfolding_result: stylish_cap
    - roll: 84
      appearance: A receipt of deposit at a bank in a far-off city
    - roll: 85
      appearance: A diary with seven missing pages
      properties:
        missing_pages: 7
    - roll: 86
      appearance: An empty silver snuffbox bearing the inscription “dreams” on its lid
    - roll: 87
      appearance: An iron holy symbol devoted to an unknown god
    - roll: 88
      appearance: A book about a legendary hero’s rise and fall, with the last chapter missing
      properties:
        final_chapter_present: false
    - roll: 89
      appearance: A vial of dragon blood
    - roll: 90
      appearance: An ancient arrow of elven design
    - roll: 91
      appearance: A needle that never bends
      properties:
        bends: false
    - roll: 92
      appearance: An ornate brooch of dwarven design
    - roll: 93
      appearance: An empty wine bottle bearing a pretty label that says, “The Wizard of Wines Winery,
        Red Dragon Crush, 331422-W”
    - roll: 94
      appearance: A mosaic tile with a multicolored, glazed surface
    - roll: 95
      appearance: A petrified mouse
    - roll: 96
      appearance: A black pirate flag adorned with a dragon’s skull and crossbones
    - roll: 97
      appearance: A tiny mechanical crab or spider that moves about when it’s not being observed
      properties:
        shape_choice:
        - crab
        - spider
        moves_when: unobserved
    - roll: 98
      appearance: A glass jar containing lard with a label that reads, “Griffon Grease”
    - roll: 99
      appearance: A wooden box with a ceramic bottom that holds a living worm with a head on each end
        of its body
      properties:
        living_worm_heads: 2
    - roll: 100
      appearance: A metal urn containing the ashes of a hero
    gm_table_use:
    - dungeon_contents
    - creature_pockets
  implementation: dnd2024:rule_trinkets
splat_contracts:
- id: dnd2024:rule_trinkets
  inputs:
    actor:
      type: participant
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - character_changes
  events: []
  capabilities:
  - character_creation
  - random_tables
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 26
  - 27
  engine_gaps: &id001
  - dnd2024:rule_trinkets
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 9d128c0a0faa32d2c06f9b81178fa0c0d9b0db87a32c63441d7ba5ba38955af2
  parent_name: Trinkets
  line_span:
    first: 2056
    count: 150
splat_clauses:
- key: source_rules
  text: '# Trinkets When you make your character, you can roll once on the Trinkets table to gain a Tiny
    trinket, a simple item lightly touched by mystery. The GM might also use this table. It can help stock
    a room in a dungeon or fill a creature’s pockets. Trinkets 1d100 Trinket 01 A mummified goblin hand
    02 A crystal that faintly glows in moonlight 03 A gold coin minted in an unknown land 04 A diary written
    in a language you don’t know 05 A brass ring that never tarnishes 06 An old chess piece made from
    glass 07 A pair of knucklebone dice, each with a skull symbol on the side that would normally show
    six pips 08 A small idol depicting a nightmarish creature that gives you unsettling dreams when you
    sleep near it 09 A lock of someone’s hair 10 The deed for a parcel of land in a realm unknown to you
    11 A 1-ounce block made from an unknown material 12 A small cloth doll skewered with needles 13 A
    tooth from an unknown beast 14 An enormous scale, perhaps from a dragon 15 A bright-green feather
    16 An old divination card bearing your likeness 17 A glass orb filled with moving smoke 18 A 1-pound
    egg with a bright-red shell 19 A pipe that blows bubbles 20 A glass jar containing a bit of flesh
    floating in pickling fluid 21 A gnome-crafted music box that plays a song you dimly remember from
    your childhood 22 A wooden statuette of a smug halfling 23 A brass orb etched with strange runes 24
    A multicolored stone disk 25 A silver icon of a raven 26 A bag containing forty-seven teeth, one of
    which is rotten 27 A shard of obsidian that always feels warm to the touch 28 A dragon’s talon strung
    on a leather necklace 29 A pair of old socks 30 A blank book whose pages refuse to hold ink, chalk,
    graphite, or any other marking 31 A silver badge that is a five-pointed star 32 A knife that belonged
    to a relative 33 A glass vial filled with nail clippings 34 A rectangular metal device with two tiny
    metal cups on one end that throws sparks when wet 1d100 Trinket 35 A white, sequined glove sized for
    a human 36 A vest with one hundred tiny pockets 37 A weightless stone 38 A sketch of a goblin 39 An
    empty glass vial that smells of perfume 40 A gemstone that looks like a lump of coal when examined
    by anyone but you 41 A scrap of cloth from an old banner 42 A rank insignia from a lost legionnaire
    43 A silver bell without a clapper 44 A mechanical canary inside a lamp 45 A miniature chest carved
    to look like it has numerous feet on the bottom 46 A dead sprite inside a clear glass bottle 47 A
    metal can that has no opening but sounds as if it is filled with liquid, sand, spiders, or broken
    glass (your choice) 48 A glass orb filled with water, in which swims a clockwork goldfish 49 A silver
    spoon with an M engraved on the handle 50 A whistle made from gold-colored wood 51 A dead scarab beetle
    the size of your hand 52 Two toy soldiers, one missing a head 53 A small box filled with different-sized
    buttons 54 A candle that can’t be lit 55 A miniature cage with no door 56 An old key 57 An indecipherable
    treasure map 58 A hilt from a broken sword 59 A rabbit’s foot 60 A glass eye 61 A cameo of a hideous
    person 62 A silver skull the size of a coin 63 An alabaster mask 64 A cone of sticky black incense
    that stinks 65 A nightcap that gives you pleasant dreams when you wear it 66 A single caltrop made
    from bone 67 A gold monocle frame without the lens 68 A 1-inch cube, each side a different color 69
    A crystal doorknob 70 A packet filled with pink dust 71 A fragment of a beautiful song, written as
    musical notes on two pieces of parchment 72 A silver teardrop earring containing a real teardrop 1d100
    Trinket 73 An eggshell painted with scenes of misery in disturbing detail 74 A fan that, when unfolded,
    shows a sleepy cat 75 A set of bone pipes 76 A four-leaf clover pressed inside a book discussing manners
    and etiquette 77 A sheet of parchment upon which is drawn a mechanical contraption 78 An ornate scabbard
    that fits no blade you have found 79 An invitation to a party where a murder happened 80 A bronze
    pentacle with an etching of a rat’s head in its center 81 A purple handkerchief embroidered with the
    name of an archmage 82 Half a floor plan for a temple, a castle, or another structure 83 A bit of
    folded cloth that, when unfolded, turns into a stylish cap 84 A receipt of deposit at a bank in a
    far-off city 85 A diary with seven missing pages 86 An empty silver snuffbox bearing the inscription
    “dreams” on its lid 87 An iron holy symbol devoted to an unknown god 88 A book about a legendary hero’s
    rise and fall, with the last chapter missing 89 A vial of dragon blood 90 An ancient arrow of elven
    design 91 A needle that never bends 92 An ornate brooch of dwarven design 93 An empty wine bottle
    bearing a pretty label that says, “The Wizard of Wines Winery, Red Dragon Crush, 331422-W” 94 A mosaic
    tile with a multicolored, glazed surface 95 A petrified mouse 96 A black pirate flag adorned with
    a dragon’s skull and crossbones 97 A tiny mechanical crab or spider that moves about when it’s not
    being observed 98 A glass jar containing lard with a label that reads, “Griffon Grease” 99 A wooden
    box with a ceramic bottom that holds a living worm with a head on each end of its body 00 A metal
    urn containing the ashes of a hero'
  sha256: 0cf6a29f82c425161f251c713057aa4cdea4fffc1328ff8b6a296f8dd621e37f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Trinkets

<!-- SRD page 26 -->

When you make your character, you can roll once\
on the Trinkets table to gain a Tiny trinket, a simple\
item lightly touched by mystery. The GM might also\
use this table. It can help stock a room in a dungeon\
or fill a creature’s pockets.\
Trinkets\
1d100 Trinket\
01 A mummified goblin hand\
02 A crystal that faintly glows in moonlight\
03 A gold coin minted in an unknown land\
04 A diary written in a language you don’t know\
05 A brass ring that never tarnishes\
06 An old chess piece made from glass\
07 A pair of knucklebone dice, each with a skull\
symbol on the side that would normally show\
six pips\
08 A small idol depicting a nightmarish creature\
that gives you unsettling dreams when you\
sleep near it\
09 A lock of someone’s hair\
10 The deed for a parcel of land in a realm un-\
known to you\
11 A 1-ounce block made from an unknown\
material\
12 A small cloth doll skewered with needles\
13 A tooth from an unknown beast\
14 An enormous scale, perhaps from a dragon\
15 A bright-green feather\
16 An old divination card bearing your likeness\
17 A glass orb filled with moving smoke\
18 A 1-pound egg with a bright-red shell\
19 A pipe that blows bubbles\
20 A glass jar containing a bit of flesh floating in\
pickling fluid\
21 A gnome-crafted music box that plays a song\
you dimly remember from your childhood\
22 A wooden statuette of a smug halfling\
23 A brass orb etched with strange runes\
24 A multicolored stone disk\
25 A silver icon of a raven\
26 A bag containing forty-seven teeth, one of\
which is rotten\
27 A shard of obsidian that always feels warm to\
the touch\
28 A dragon’s talon strung on a leather necklace\
29 A pair of old socks\
30 A blank book whose pages refuse to hold ink,\
chalk, graphite, or any other marking\
31 A silver badge that is a five-pointed star\
32 A knife that belonged to a relative\
33 A glass vial filled with nail clippings\
34 A rectangular metal device with two tiny metal\
cups on one end that throws sparks when wet\

<!-- SRD page 27 -->

1d100 Trinket\
35 A white, sequined glove sized for a human\
36 A vest with one hundred tiny pockets\
37 A weightless stone\
38 A sketch of a goblin\
39 An empty glass vial that smells of perfume\
40 A gemstone that looks like a lump of coal when\
examined by anyone but you\
41 A scrap of cloth from an old banner\
42 A rank insignia from a lost legionnaire\
43 A silver bell without a clapper\
44 A mechanical canary inside a lamp\
45 A miniature chest carved to look like it has nu-\
merous feet on the bottom\
46 A dead sprite inside a clear glass bottle\
47 A metal can that has no opening but sounds as\
if it is filled with liquid, sand, spiders, or broken\
glass (your choice)\
48 A glass orb filled with water, in which swims a\
clockwork goldfish\
49 A silver spoon with an M engraved on the handle\
50 A whistle made from gold-colored wood\
51 A dead scarab beetle the size of your hand\
52 Two toy soldiers, one missing a head\
53 A small box filled with different-sized buttons\
54 A candle that can’t be lit\
55 A miniature cage with no door\
56 An old key\
57 An indecipherable treasure map\
58 A hilt from a broken sword\
59 A rabbit’s foot\
60 A glass eye\
61 A cameo of a hideous person\
62 A silver skull the size of a coin\
63 An alabaster mask\
64 A cone of sticky black incense that stinks\
65 A nightcap that gives you pleasant dreams\
when you wear it\
66 A single caltrop made from bone\
67 A gold monocle frame without the lens\
68 A 1-inch cube, each side a different color\
69 A crystal doorknob\
70 A packet filled with pink dust\
71 A fragment of a beautiful song, written as mu-\
sical notes on two pieces of parchment\
72 A silver teardrop earring containing a real\
teardrop\
1d100 Trinket\
73 An eggshell painted with scenes of misery in\
disturbing detail\
74 A fan that, when unfolded, shows a sleepy cat\
75 A set of bone pipes\
76 A four-leaf clover pressed inside a book dis-\
cussing manners and etiquette\
77 A sheet of parchment upon which is drawn a\
mechanical contraption\
78 An ornate scabbard that fits no blade you have\
found\
79 An invitation to a party where a murder\
happened\
80 A bronze pentacle with an etching of a rat’s\
head in its center\
81 A purple handkerchief embroidered with the\
name of an archmage\
82 Half a floor plan for a temple, a castle, or an-\
other structure\
83 A bit of folded cloth that, when unfolded, turns\
into a stylish cap\
84 A receipt of deposit at a bank in a far-off city\
85 A diary with seven missing pages\
86 An empty silver snuffbox bearing the inscrip-\
tion “dreams” on its lid\
87 An iron holy symbol devoted to an unknown\
god\
88 A book about a legendary hero’s rise and fall,\
with the last chapter missing\
89 A vial of dragon blood\
90 An ancient arrow of elven design\
91 A needle that never bends\
92 An ornate brooch of dwarven design\
93 An empty wine bottle bearing a pretty label\
that says, “The Wizard of Wines Winery, Red\
Dragon Crush, 331422-W”\
94 A mosaic tile with a multicolored, glazed\
surface\
95 A petrified mouse\
96 A black pirate flag adorned with a dragon’s\
skull and crossbones\
97 A tiny mechanical crab or spider that moves\
about when it’s not being observed\
98 A glass jar containing lard with a label that\
reads, “Griffon Grease”\
99 A wooden box with a ceramic bottom that\
holds a living worm with a head on each end of\
its body\
00 A metal urn containing the ashes of a hero\
