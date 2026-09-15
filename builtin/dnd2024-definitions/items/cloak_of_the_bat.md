---
splat_schema: 1
splat_id: e9e47748-b495-55bd-af8b-7585b3209dfe
splat_kind: item
splat_key: dnd2024:items/cloak_of_the_bat
splat_name: Cloak of the Bat
splat_origin:
  key: dnd2024:reference/magic_item/cloak_of_the_bat
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
  mechanic: dnd2024:items/cloak_of_the_bat
  implementation: dnd2024:item_cloak_of_the_bat
  parameters:
    equipped: worn
    checks:
    - ability: dexterity
      skill: stealth
      mode: advantage
    flight:
      speed_ft: 40
      requires_all:
      - light_level_in:
        - dim_light
        - darkness
      - gripping_cloak_edges: true
      ends_immediately_if_requirement_lost: true
    spells:
    - spell:
        key: dnd2024:spells/polymorph
      target: self
      form:
        key: dnd2024:creatures/bat
      retain_ability_scores:
      - intelligence
      - wisdom
      - charisma
      activation_requires_light:
      - dim_light
      - darkness
      uses:
        maximum: 1
        recovery_event: time.dawn
    attunement_required: true
splat_contracts:
- id: dnd2024:item_cloak_of_the_bat
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
  - ability_check.pending
  - lighting.changed
  - item.grip_changed
  - spell.cast_pending
  - time.dawn
  - item.attunement_completed
  - item.attunement_ended
  capabilities:
  - item_effects
  - item_spellcasting
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: d347b2d8eeb2ebedf22981142d2d627e42eefa9a86b301eed318a8f95334e302
  source_path: builtin/srd-5.2.1/magic_item/cloak_of_the_bat.md
  pages:
  - 216
  engine_gaps: &id001
  - dnd2024:item_cloak_of_the_bat
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_3.md
  specification_sha256: b304e19839dc0ac5a258626ab3f613542098d82d9b2a8e228bde3607a32f3b84
splat_clauses:
- key: source_rules
  text: '# Cloak of the Bat Wondrous Item, Rare (Requires Attunement) While wearing this cloak, you have
    Advantage on Dexterity (Stealth) checks. In an area of Dim Light or Darkness, you can grip the edges
    of the cloak and use it to gain a Fly Speed of 40 feet. If you ever fail to grip the cloak’s edges
    while flying in this way, or if you are no longer in Dim Light or Darkness, you lose this Fly Speed.
    While wearing the cloak in an area of Dim Light or Darkness, you can cast Polymorph on yourself, shape-shifting
    into a Bat. While in that form, you retain your Intelligence, Wisdom, and Charisma scores. The cloak
    can’t be used this way again until the next dawn.'
  sha256: 418d3e2c1cd47f62dc9bf2d137b5274b8e2e71989125fff7b3e51f3123a472f8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cloak of the Bat

<!-- SRD page 216 -->

Wondrous Item, Rare (Requires Attunement)\
While wearing this cloak, you have Advantage on\
Dexterity (Stealth) checks. In an area of Dim Light\
or Darkness, you can grip the edges of the cloak and\
use it to gain a Fly Speed of 40 feet. If you ever fail\
to grip the cloak’s edges while flying in this way, or\
if you are no longer in Dim Light or Darkness, you\
lose this Fly Speed.\
While wearing the cloak in an area of Dim Light\
or Darkness, you can cast Polymorph on yourself,\
shape-shifting into a Bat. While in that form, you\
retain your Intelligence, Wisdom, and Charisma\
scores. The cloak can’t be used this way again until\
the next dawn.\

Original layout: [[../../srd-5.2.1/pages/216|page 216]].
