---
splat_schema: 1
splat_id: 7d079e82-c907-5bf5-bfb9-5e774668e738
splat_kind: item
splat_key: dnd2024:items/weapon_of_warning
splat_name: Weapon of Warning
splat_origin:
  key: dnd2024:reference/magic_item/weapon_of_warning
splat_sources:
- title: SRD 5.2.1
  locator: pp. 253
  url: https://www.dndbeyond.com/srd
splat_item:
  category: weapon
  rarity: uncommon
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/weapon_of_warning
  implementation: dnd2024:item_weapon_of_warning
  parameters:
    equipment_state: within_user_reach
    aura:
      radius_ft: 30
      origin: user
      targets:
      - self
      - allies
      combat_start:
        wake_natural_sleep: true
        wake_magical_sleep: false
      initiative_advantage: true
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_weapons
splat_contracts:
- id: dnd2024:item_weapon_of_warning
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
  - combat.started
  - initiative.pending
  - position.changed
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - spatial_state
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 54cc403316414175fff7256b71f6915945411fe364e30e0c02ebeee7a94fbde6
  source_path: builtin/srd-5.2.1/magic_item/weapon_of_warning.md
  pages:
  - 253
  engine_gaps: &id001
  - dnd2024:item_weapon_of_warning
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 7639d66d2c50ff770b7711f2aa15c0cd7adc20870dcbe12fe9d20bf9c1b5bbca
splat_clauses:
- key: source_rules
  text: '# Weapon of Warning Weapon (Any Simple or Martial), Uncommon (Requires Attunement) As long as
    this weapon is within your reach and you are attuned to it, you and allies within 30 feet of you gain
    the following benefits. Alarm. The weapon magically awakens each subject who is sleeping naturally
    when combat begins. This benefit doesn’t wake a subject from magically induced sleep. Supernatural
    Readiness. Each subject has Advantage on its Initiative rolls.'
  sha256: 27d6ae9d1115d97fe643d2610e52b239091e9ee90f6002c4e8db4f810fe28e8e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Weapon of Warning

<!-- SRD page 253 -->

Weapon (Any Simple or Martial), Uncommon (Requires\
Attunement)\
As long as this weapon is within your reach and you\
are attuned to it, you and allies within 30 feet of you\
gain the following benefits.\
Alarm. The weapon magically awakens each sub-\
ject who is sleeping naturally when combat begins.\
This benefit doesn’t wake a subject from magically\
induced sleep.\
Supernatural Readiness. Each subject has Advan-\
tage on its Initiative rolls.\

Original layout: [[../../srd-5.2.1/pages/253|page 253]].
