---
splat_schema: 1
splat_id: 3d74ea97-d180-5a06-881c-f9d30d23f1ad
splat_kind: item
splat_key: dnd2024:items/cube_of_force
splat_name: Cube of Force
splat_origin:
  key: dnd2024:reference/magic_item/cube_of_force
splat_sources:
- title: SRD 5.2.1
  locator: pp. 216
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/cube_of_force
  implementation: dnd2024:item_cube_of_force
  parameters:
    dimensions_inches:
    - 1
    - 1
    - 1
    activation: press_spell_face
    charges:
      initial: 10
      maximum: 10
      recovery:
        event: time.dawn
        amount:
          roll:
            count: 1
            sides: 6
        cap_at_maximum: true
    spells:
    - spell:
        key: dnd2024:spells/mage_armor
      charge_cost: 1
      save_dc: 17
    - spell:
        key: dnd2024:spells/shield
      charge_cost: 1
      save_dc: 17
    - spell:
        key: dnd2024:spells/tiny_hut
      charge_cost: 3
      save_dc: 17
    - spell:
        key: dnd2024:spells/private_sanctum
      charge_cost: 4
      save_dc: 17
    - spell:
        key: dnd2024:spells/resilient_sphere
      charge_cost: 4
      save_dc: 17
    - spell:
        key: dnd2024:spells/wall_of_force
      charge_cost: 5
      save_dc: 17
    attunement_required: true
splat_contracts:
- id: dnd2024:item_cube_of_force
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    selection:
      required: false
      type: choice
    context:
      required: false
      type: resolution
  outputs:
  - effects
  - item_state_changes
  events:
  - spell.cast_pending
  - time.dawn
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - dice
  - item_spellcasting
  - item_charges
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 38372e4226acc6630cf1b61f2065079f3d1007f3117ff1e1ec40ec90a3aa02aa
  source_path: builtin/srd-5.2.1/magic_item/cube_of_force.md
  pages:
  - 216
  engine_gaps: &id001
  - dnd2024:item_cube_of_force
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: e5d19aa17b9f0c45c534bb77b91fbd77969fc1fc17fb3a098bda761c93fe5cfc
splat_clauses:
- key: source_rules
  text: '# Cube of Force Wondrous Item, Rare (Requires Attunement) This cube is about an inch across.
    Each face has a distinct marking on it. You can press one of those faces, expend the number of charges
    required for it, and thereby cast the spell associated with it (save DC 17), as shown in the Cube
    of Force Faces table. The cube starts with 10 charges, and it regains 1d6 expended charges daily at
    dawn. Cube of Force Faces Spell Charge Cost Mage Armor 1 Shield 1 Tiny Hut 3 Private Sanctum 4 Resilient
    Sphere 4 Wall of Force 5'
  sha256: af49b83a2865a91d4f96ad426c2353a32097edfe1bcb18c0fb5e4a73f47c2121
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cube of Force

<!-- SRD page 216 -->

Wondrous Item, Rare (Requires Attunement)\
This cube is about an inch across. Each face has a\
distinct marking on it. You can press one of those\
faces, expend the number of charges required for it,\
and thereby cast the spell associated with it (save\
DC 17), as shown in the Cube of Force Faces table.\
The cube starts with 10 charges, and it regains\
1d6 expended charges daily at dawn.\
Cube of Force Faces\
Spell Charge Cost\
Mage Armor 1\
Shield 1\
Tiny Hut 3\
Private Sanctum 4\
Resilient Sphere 4\
Wall of Force 5\

Original layout: [[../../srd-5.2.1/pages/216|page 216]].
