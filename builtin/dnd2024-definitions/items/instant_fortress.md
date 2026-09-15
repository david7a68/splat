---
splat_schema: 1
splat_id: 23e0b3bc-2f00-5740-b0c8-97b445edede9
splat_kind: item
splat_key: dnd2024:items/instant_fortress
splat_name: Instant Fortress
splat_origin:
  key: dnd2024:reference/magic_item/instant_fortress
splat_sources:
- title: SRD 5.2.1
  locator: pp. 227
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/instant_fortress
  implementation: dnd2024:item_instant_fortress
  parameters:
    statuette_size_inches: 1
    material: adamantine
    expand:
      action: magic
      place_on_ground: true
      command_word: true
      displace:
        creatures: unoccupied_adjacent_outside_space
        objects: outside_tower
        exclude_worn_carried_objects: true
    shrink:
      command_word: true
      requires: tower_empty
      preserve_damage: true
    tower:
      width_ft: 20
      length_ft: 20
      height_ft: 30
      arrow_slits: all_sides
      battlement: true
      floors: 2
      connector:
        chooser: user
        options:
        - ladder
        - staircase
        - ramp
      roof_access: trapdoor
      ground_door:
        faces: user_at_creation
        open:
          actor: attuned_user
          action: bonus_action
          command: true
        immune_to:
        - key: dnd2024:spells/knock
        immune_to_similar_opening_magic: true
      cannot_tip_over: true
      component_statistics:
        components:
        - roof
        - door
        - walls
        ac: 20
        hp: 100
        immunity:
        - bludgeoning
        - piercing
        - slashing
        immunity_exception: siege_equipment
        resistance: all_other_damage
    repair:
      only_spell:
        key: dnd2024:spells/wish
      counts_as: replicate_spell_level_8_or_lower
      hp_restored: all
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
    - key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:item_instant_fortress
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
  - command.issued
  - spell.cast_completed
  - damage.pending
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - spellcasting
  - roll_resolution
  - spatial_state
  - participant_state
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 8e6f33e5172874b61a3c68c2631a8b2d4a67718a42accb27d4b7e524cd7eea7c
  source_path: builtin/srd-5.2.1/magic_item/instant_fortress.md
  pages:
  - 227
  engine_gaps: &id001
  - dnd2024:item_instant_fortress
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 305c94be654ba10cac888f4c57aa7047a832d353936a56de1bea8b737c28c2e9
splat_clauses:
- key: source_rules
  text: '# Instant Fortress Wondrous Item, Rare (Requires Attunement) As a Magic action, you can place
    this 1-inch adamantine statuette on the ground and, using a command word, cause it to grow rapidly
    into a square adamantine tower. Repeating the command word causes the tower to revert to statuette
    form, which works only if the tower is empty. Each creature in the area where the tower appears is
    pushed to an unoccupied space outside but next to the tower. Objects in the area that aren’t being
    worn or carried are also pushed clear of the tower. The tower is 20 feet on a side and 30 feet high,
    with arrow slits on all sides and a battlement atop it. Its interior is divided into two floors, with
    a ladder, staircase, or ramp (your choice) connecting them. This ladder, staircase, or ramp ends at
    a trapdoor leading to the roof. When created, the tower has a single door at ground level on the side
    facing you. The door opens only at your command, which you can issue as a Bonus Action. It is immune
    to the Knock spell and similar magic. Magic prevents the tower from being tipped over. The roof, the
    door, and the walls each have AC 20; HP 100; Immunity to Bludgeoning, Piercing, and Slashing damage
    except that which is dealt by siege equipment; and Resistance to all other damage. Shrinking the tower
    back down to statuette form doesn’t repair damage to the tower. Only a Wish spell can repair the tower
    (this use of the spell counts as replicating a spell of level 8 or lower). Each casting of Wish causes
    the tower to regain all its Hit Points.'
  sha256: e98727bada481b579ae11821ed621f8d9899096d3f0a035ebc1d6b9c706da750
  status: contract_specified
splat_requires_mechanics: *id001
---

# Instant Fortress

<!-- SRD page 227 -->

Wondrous Item, Rare (Requires Attunement)\
As a Magic action, you can place this 1-inch\
adamantine statuette on the ground and, using\
a command word, cause it to grow rapidly into a\
square adamantine tower. Repeating the command\
word causes the tower to revert to statuette form,\
which works only if the tower is empty. Each crea-\
ture in the area where the tower appears is pushed\
to an unoccupied space outside but next to the\
tower. Objects in the area that aren’t being worn or\
carried are also pushed clear of the tower.\
The tower is 20 feet on a side and 30 feet high,\
with arrow slits on all sides and a battlement atop\
it. Its interior is divided into two floors, with a lad-\
der, staircase, or ramp (your choice) connecting\
them. This ladder, staircase, or ramp ends at a trap-\
door leading to the roof. When created, the tower\
has a single door at ground level on the side facing\
you. The door opens only at your command, which\
you can issue as a Bonus Action. It is immune to the\
Knock spell and similar magic.\
Magic prevents the tower from being tipped over.\
The roof, the door, and the walls each have AC 20;\
HP 100; Immunity to Bludgeoning, Piercing, and\
Slashing damage except that which is dealt by siege\
equipment; and Resistance to all other damage.\
Shrinking the tower back down to statuette form\
doesn’t repair damage to the tower. Only a Wish\
spell can repair the tower (this use of the spell\
counts as replicating a spell of level 8 or lower).\
Each casting of Wish causes the tower to regain all\
its Hit Points.\

Original layout: [[../../srd-5.2.1/pages/227|page 227]].
