---
splat_schema: 1
splat_id: 12a23b10-d2a5-5f8f-b36c-8c5fe9570cb3
splat_kind: item
splat_key: dnd2024:items/deck_of_illusions
splat_name: Deck of Illusions
splat_origin:
  key: dnd2024:reference/magic_item/deck_of_illusions
splat_sources:
- title: SRD 5.2.1
  locator: pp. 217, 218
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/deck_of_illusions
  implementation: dnd2024:item_deck_of_illusions
  parameters:
    full_deck:
      cards: 34
      specific_creatures: 32
      mirrored_cards: 2
    found_deck_missing:
      expression: 1d20-1
      scope: use
    activate:
      action: magic
      draw: random_only
      throw_range_ft: 30
      illusion:
        roll:
          expression: 1d100
          scope: use
        rows:
        - min: 1
          max: 3
          result:
            appearance:
              key: dnd2024:creatures/adult_red_dragon
        - min: 4
          max: 6
          result:
            appearance:
              key: dnd2024:creatures/archmage
        - min: 7
          max: 9
          result:
            appearance:
              key: dnd2024:creatures/assassin
        - min: 10
          max: 12
          result:
            appearance:
              key: dnd2024:creatures/bandit_captain
        - min: 13
          max: 15
          result:
            appearance:
              key: dnd2024:creatures/basilisk
        - min: 16
          max: 18
          result:
            appearance:
              key: dnd2024:creatures/berserker
        - min: 19
          max: 21
          result:
            appearance:
              key: dnd2024:creatures/bugbear_warrior
        - min: 22
          max: 24
          result:
            appearance:
              key: dnd2024:creatures/cloud_giant
        - min: 25
          max: 27
          result:
            appearance:
              key: dnd2024:creatures/druid
        - min: 28
          max: 30
          result:
            appearance:
              key: dnd2024:creatures/erinyes
        - min: 31
          max: 33
          result:
            appearance:
              key: dnd2024:creatures/ettin
        - min: 34
          max: 36
          result:
            appearance:
              key: dnd2024:creatures/fire_giant
        - min: 37
          max: 39
          result:
            appearance:
              key: dnd2024:creatures/frost_giant
        - min: 40
          max: 42
          result:
            appearance:
              key: dnd2024:creatures/gnoll_warrior
        - min: 43
          max: 45
          result:
            appearance:
              key: dnd2024:creatures/goblin_warrior
        - min: 46
          max: 48
          result:
            appearance:
              key: dnd2024:creatures/guardian_naga
        - min: 49
          max: 51
          result:
            appearance:
              key: dnd2024:creatures/hill_giant
        - min: 52
          max: 54
          result:
            appearance:
              key: dnd2024:creatures/hobgoblin_warrior
        - min: 55
          max: 57
          result:
            appearance:
              key: dnd2024:creatures/incubus
        - min: 58
          max: 60
          result:
            appearance:
              key: dnd2024:creatures/iron_golem
        - min: 61
          max: 63
          result:
            appearance:
              key: dnd2024:creatures/knight
        - min: 64
          max: 66
          result:
            appearance:
              key: dnd2024:creatures/kobold_warrior
        - min: 67
          max: 69
          result:
            appearance:
              key: dnd2024:creatures/lich
        - min: 70
          max: 72
          result:
            appearance:
              key: dnd2024:creatures/medusa
        - min: 73
          max: 75
          result:
            appearance:
              key: dnd2024:creatures/night_hag
        - min: 76
          max: 78
          result:
            appearance:
              key: dnd2024:creatures/ogre
        - min: 79
          max: 81
          result:
            appearance:
              key: dnd2024:creatures/oni
        - min: 82
          max: 84
          result:
            appearance:
              key: dnd2024:creatures/priest
        - min: 85
          max: 87
          result:
            appearance:
              key: dnd2024:creatures/succubus
        - min: 88
          max: 90
          result:
            appearance:
              key: dnd2024:creatures/troll
        - min: 91
          max: 93
          result:
            appearance:
              key: dnd2024:creatures/warrior_veteran
        - min: 94
          max: 96
          result:
            appearance:
              key: dnd2024:creatures/wyvern
        - min: 97
          max: 100
          result:
            appearance: card_drawer
    illusion:
      behavior: imitate_creature
      can_harm: false
      move:
        action: magic
        user_maximum_distance_ft: 120
        user_must_see: true
        destination_maximum_distance_from_card_ft: 30
      physical_interaction:
        objects_pass_through: true
        reveals_false: true
      inspect:
        action: study
        check:
          ability: intelligence
          skill: investigation
          dc: 15
        success: identify_illusion
      ends_on:
      - card_moved
      - dispelled
      dispel_examples:
      - key: dnd2024:spells/dispel_magic
      on_end:
        card_image: disappears
        card_reusable: false
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_deck_of_illusions
  inputs:
    actor:
      type: participant
      required: true
    source_item:
      type: item_instance
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    choice:
      required: false
      type: choice
    event:
      required: false
      type: event
    campaign:
      required: false
      type: campaign
  outputs:
  - item_state
  - effects
  - resolutions
  events:
  - action.magic_pending
  - action.study_pending
  - object.interacted
  - item.moved
  - effect.dispelled
  capabilities:
  - inventory
  - effects
  - spellcasting
  - roll_resolution
  - participant_state
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: cdeb81080556a39e21f187b2fbae94b7bb6ac18de07cfc0b98a948fe6e57ac85
  source_path: builtin/srd-5.2.1/magic_item/deck_of_illusions.md
  pages:
  - 217
  - 218
  engine_gaps: &id001
  - dnd2024:item_deck_of_illusions
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 5c6c2925cc49886509501911f4a8f14e3f77b1bded9f27ff515c7bee72fe9517
splat_clauses:
- key: source_rules
  text: '# Deck of Illusions Wondrous Item, Uncommon This box contains a set of cards. A full deck has
    34 cards: 32 depicting specific creatures and two with a mirrored surface. A deck found as treasure
    is usually missing 1d20 - 1 cards. The magic of the deck functions only if its cards are drawn at
    random. You can take a Magic action to draw a card at random from the deck and throw it to the ground
    at a point within 30 feet of yourself. An illusion of a creature, determined by rolling on the Deck
    of Illusions table, forms over the thrown card and remains until dispelled. The illusory creature
    created by the card looks and behaves like a real creature of its kind, except that it can do no harm.
    While you are within 120 feet of the illusory creature and can see it, you can take a Magic action
    to move it anywhere within 30 feet of its card. Any physical interaction with the illusory creature
    reveals it to be false, because objects pass through it. A creature that takes a Study action to visually
    inspect the illusory creature identifies it as an illusion with a successful DC 15 Intelligence (Investigation)
    check. The illusion lasts until its card is moved or the illusion is dispelled (using a Dispel Magic
    spell or a similar effect). When the illusion ends, the image on its card disappears, and that card
    can’t be used again. Deck of Illusions 1d100 Illusion* 01–03 Adult Red Dragon 04–06 Archmage 07–09
    Assassin 10–12 Bandit Captain 13–15 Basilisk 16–18 Berserker 19–21 Bugbear Warrior 22–24 Cloud Giant
    25–27 Druid 28–30 Erinyes 31–33 Ettin 34–36 Fire Giant 37–39 Frost Giant 40–42 Gnoll Warrior 43–45
    Goblin Warrior 46–48 Guardian Naga 49–51 Hill Giant 52–54 Hobgoblin Warrior 55–57 Incubus 58–60 Iron
    Golem 61–63 Knight 64–66 Kobold Warrior 67–69 Lich 70–72 Medusa 73–75 Night Hag 76–78 Ogre 79–81 Oni
    1d100 Illusion* 82–84 Priest 85–87 Succubus 88–90 Troll 91–93 Veteran Warrior 94–96 Wyvern 97–00 The
    card drawer *Stat blocks for these creatures (except the card drawer) appear in “Monsters.”'
  sha256: 5ffdff58e13dcd973737da085275b0dbb26ba17927dda235ba75dcded2caea41
  status: contract_specified
splat_requires_mechanics: *id001
---

# Deck of Illusions

<!-- SRD page 217 -->

Wondrous Item, Uncommon\
This box contains a set of cards. A full deck has 34\
cards: 32 depicting specific creatures and two with\
a mirrored surface. A deck found as treasure is usu-\
ally missing 1d20 − 1 cards.\
The magic of the deck functions only if its cards\
are drawn at random. You can take a Magic action\
to draw a card at random from the deck and throw\
it to the ground at a point within 30 feet of yourself.\
An illusion of a creature, determined by rolling on\
the Deck of Illusions table, forms over the thrown\
card and remains until dispelled. The illusory crea-\
ture created by the card looks and behaves like a\
real creature of its kind, except that it can do no\
harm. While you are within 120 feet of the illusory\
creature and can see it, you can take a Magic action\
to move it anywhere within 30 feet of its card.\
Any physical interaction with the illusory crea-\
ture reveals it to be false, because objects pass\
through it. A creature that takes a Study action to\
visually inspect the illusory creature identifies it as\
an illusion with a successful DC 15 Intelligence (In-\
vestigation) check. The illusion lasts until its card\
is moved or the illusion is dispelled (using a Dispel\
Magic spell or a similar effect). When the illusion\
ends, the image on its card disappears, and that\
card can’t be used again.\
Deck of Illusions\
1d100 Illusion*\
01–03 Adult Red Dragon\
04–06 Archmage\
07–09 Assassin\
10–12 Bandit Captain\
13–15 Basilisk\
16–18 Berserker\
19–21 Bugbear Warrior\
22–24 Cloud Giant\
25–27 Druid\
28–30 Erinyes\
31–33 Ettin\
34–36 Fire Giant\
37–39 Frost Giant\
40–42 Gnoll Warrior\
43–45 Goblin Warrior\
46–48 Guardian Naga\
49–51 Hill Giant\
52–54 Hobgoblin Warrior\
55–57 Incubus\
58–60 Iron Golem\
61–63 Knight\
64–66 Kobold Warrior\
67–69 Lich\
70–72 Medusa\
73–75 Night Hag\
76–78 Ogre\
79–81 Oni\

<!-- SRD page 218 -->

1d100 Illusion*\
82–84 Priest\
85–87 Succubus\
88–90 Troll\
91–93 Veteran Warrior\
94–96 Wyvern\
97–00 The card drawer\
*Stat blocks for these creatures (except the card drawer) appear in\
“Monsters.”\

Original layout: [[../../srd-5.2.1/pages/217|page 217]], [[../../srd-5.2.1/pages/218|page 218]].
