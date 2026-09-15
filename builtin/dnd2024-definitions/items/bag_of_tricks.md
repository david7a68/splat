---
splat_schema: 1
splat_id: e0b35dea-1113-5224-abe1-792ab8accccf
splat_kind: item
splat_key: dnd2024:items/bag_of_tricks
splat_name: Bag of Tricks
splat_origin:
  key: dnd2024:reference/magic_item/bag_of_tricks
splat_sources:
- title: SRD 5.2.1
  locator: pp. 212
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: none
splat_rule:
  mechanic: dnd2024:items/bag_of_tricks
  implementation: dnd2024:item_bag_of_tricks
  parameters:
    uses:
      maximum: 3
      initial: 3
      recovery:
        event: dawn
        amount:
          expression: '3'
          scope: use
        cap: maximum
    activate:
      action: magic
      throw_maximum_ft: 20
      draw_and_throw: true
    variants:
      gray:
        roll:
          expression: 1d8
          scope: use
        rows:
        - min: 1
          max: 1
          result:
            creature:
              key: dnd2024:creatures/weasel
        - min: 2
          max: 2
          result:
            creature:
              key: dnd2024:creatures/giant_rat
        - min: 3
          max: 3
          result:
            creature:
              key: dnd2024:creatures/badger
        - min: 4
          max: 4
          result:
            creature:
              key: dnd2024:creatures/boar
        - min: 5
          max: 5
          result:
            creature:
              key: dnd2024:creatures/panther
        - min: 6
          max: 6
          result:
            creature:
              key: dnd2024:creatures/giant_badger
        - min: 7
          max: 7
          result:
            creature:
              key: dnd2024:creatures/dire_wolf
        - min: 8
          max: 8
          result:
            creature:
              key: dnd2024:creatures/giant_elk
      rust:
        roll:
          expression: 1d8
          scope: use
        rows:
        - min: 1
          max: 1
          result:
            creature:
              key: dnd2024:creatures/rat
        - min: 2
          max: 2
          result:
            creature:
              key: dnd2024:creatures/owl
        - min: 3
          max: 3
          result:
            creature:
              key: dnd2024:creatures/mastiff
        - min: 4
          max: 4
          result:
            creature:
              key: dnd2024:creatures/goat
        - min: 5
          max: 5
          result:
            creature:
              key: dnd2024:creatures/giant_goat
        - min: 6
          max: 6
          result:
            creature:
              key: dnd2024:creatures/giant_boar
        - min: 7
          max: 7
          result:
            creature:
              key: dnd2024:creatures/lion
        - min: 8
          max: 8
          result:
            creature:
              key: dnd2024:creatures/brown_bear
      tan:
        roll:
          expression: 1d8
          scope: use
        rows:
        - min: 1
          max: 1
          result:
            creature:
              key: dnd2024:creatures/jackal
        - min: 2
          max: 2
          result:
            creature:
              key: dnd2024:creatures/ape
        - min: 3
          max: 3
          result:
            creature:
              key: dnd2024:creatures/baboon
        - min: 4
          max: 4
          result:
            creature:
              key: dnd2024:creatures/axe_beak
        - min: 5
          max: 5
          result:
            creature:
              key: dnd2024:creatures/black_bear
        - min: 6
          max: 6
          result:
            creature:
              key: dnd2024:creatures/giant_weasel
        - min: 7
          max: 7
          result:
            creature:
              key: dnd2024:creatures/giant_hyena
        - min: 8
          max: 8
          result:
            creature:
              key: dnd2024:creatures/tiger
    summon:
      attitude: friendly_to_user_and_allies
      initiative: immediately_after_user_same_count
      end:
      - next_dawn
      - zero_hit_points
      command:
        action: bonus_action
        controls:
        - next_turn_movement
        - next_turn_action
      without_command: nature_appropriate_behavior
    attunement:
      required: false
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
splat_contracts:
- id: dnd2024:item_bag_of_tricks
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
  - action.bonus_pending
  - time.dawn
  - participant.hp_changed
  capabilities:
  - inventory
  - effects
  - resources_and_time
  - roll_resolution
  - spatial_state
  - participant_state
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 2dbfc1e06e1bd975cb8068b2df38a5536131ede357be16aff9fff5b159e970fb
  source_path: builtin/srd-5.2.1/magic_item/bag_of_tricks.md
  pages:
  - 212
  engine_gaps: &id001
  - dnd2024:item_bag_of_tricks
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 636668dd52d10a36b47ca019405e249197c67416eaf5b3420bd5365a9c57bb29
splat_clauses:
- key: source_rules
  text: '# Bag of Tricks Wondrous Item, Uncommon This bag made from gray, rust, or tan cloth appears empty.
    Reaching inside the bag, however, reveals the presence of a small, fuzzy object. You can take a Magic
    action to pull the fuzzy object from the bag and throw it up to 20 feet. When the object lands, it
    transforms into a creature you determine by rolling on the table that corresponds to the bag’s color.
    See “Monsters” for the creature’s stat block. The creature vanishes at the next dawn or when it is
    reduced to 0 Hit Points. The creature is Friendly to you and your allies, and it acts immediately
    after you on your Initiative count. You can take a Bonus Action to command how the creature moves
    and what action it takes on its next turn, such as attacking an enemy. In the absence of such orders,
    the creature acts in a fashion appropriate to its nature. Once three fuzzy objects have been pulled
    from the bag, the bag can’t be used again until the next dawn. Gray Bag of Tricks 1d8 Creature 1d8
    Creature 1 Weasel 5 Panther 2 Giant Rat 6 Giant Badger 3 Badger 7 Dire Wolf 4 Boar 8 Giant Elk Rust
    Bag of Tricks 1d8 Creature 1d8 Creature 1 Rat 5 Giant Goat 2 Owl 6 Giant Boar 3 Mastiff 7 Lion 4 Goat
    8 Brown Bear Tan Bag of Tricks 1d8 Creature 1d8 Creature 1 Jackal 5 Black Bear 2 Ape 6 Giant Weasel
    3 Baboon 7 Giant Hyena 4 Axe Beak 8 Tiger'
  sha256: ebe3f976cb50ef8364b06214be9402556d594bd632b49b9bd2f2b503d25d9198
  status: contract_specified
splat_requires_mechanics: *id001
---

# Bag of Tricks

<!-- SRD page 212 -->

Wondrous Item, Uncommon\
This bag made from gray, rust, or tan cloth appears\
empty. Reaching inside the bag, however, reveals\
the presence of a small, fuzzy object.\
You can take a Magic action to pull the fuzzy ob-\
ject from the bag and throw it up to 20 feet. When\
the object lands, it transforms into a creature you\
determine by rolling on the table that corresponds\
to the bag’s color. See “Monsters” for the creature’s\
stat block. The creature vanishes at the next dawn\
or when it is reduced to 0 Hit Points.\
The creature is Friendly to you and your allies,\
and it acts immediately after you on your Initiative\
count. You can take a Bonus Action to command\
how the creature moves and what action it takes on\
its next turn, such as attacking an enemy. In the ab-\
sence of such orders, the creature acts in a fashion\
appropriate to its nature.\
Once three fuzzy objects have been pulled from\
the bag, the bag can’t be used again until the next\
dawn.\
Gray Bag of Tricks\
1d8 Creature 1d8 Creature\
1 Weasel 5 Panther\
2 Giant Rat 6 Giant Badger\
3 Badger 7 Dire Wolf\
4 Boar 8 Giant Elk\
Rust Bag of Tricks\
1d8 Creature 1d8 Creature\
1 Rat 5 Giant Goat\
2 Owl 6 Giant Boar\
3 Mastiff 7 Lion\
4 Goat 8 Brown Bear\
Tan Bag of Tricks\
1d8 Creature 1d8 Creature\
1 Jackal 5 Black Bear\
2 Ape 6 Giant Weasel\
3 Baboon 7 Giant Hyena\
4 Axe Beak 8 Tiger\

Original layout: [[../../srd-5.2.1/pages/212|page 212]].
