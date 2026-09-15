---
splat_schema: 1
splat_id: 9ee9164a-1319-5110-b960-906302f7f1cd
splat_kind: item
splat_key: dnd2024:items/bracers_of_archery
splat_name: Bracers of Archery
splat_origin:
  key: dnd2024:reference/magic_item/bracers_of_archery
splat_sources:
- title: SRD 5.2.1
  locator: pp. 214
  url: https://www.dndbeyond.com/srd
splat_item:
  category: wondrous_item
  rarity: uncommon
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/bracers_of_archery
  implementation: dnd2024:item_bracers_of_archery
  parameters:
    equipment_state: worn
    weapon_proficiencies:
    - key: dnd2024:equipment/longbow
    - key: dnd2024:equipment/shortbow
    damage_bonus:
      weapons:
      - key: dnd2024:equipment/longbow
      - key: dnd2024:equipment/shortbow
      amount: 2
    attunement:
      required: true
    common_rules:
    - key: dnd2024:rules/magic_item_categories_wondrous_items
splat_contracts:
- id: dnd2024:item_bracers_of_archery
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
  - equipment.changed
  - attack.damage_pending
  - attunement.changed
  capabilities:
  - inventory
  - effects
  - roll_resolution
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: a91967486efeb3bf3e92f7168f392dda96d2e8eb406d234c1e859710053932fc
  source_path: builtin/srd-5.2.1/magic_item/bracers_of_archery.md
  pages:
  - 214
  engine_gaps: &id001
  - dnd2024:item_bracers_of_archery
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_2.md
  specification_sha256: 08bf00895ea6d19d287ceadfc9130da5c5709b03c01f5dd25b53a1a08517d901
splat_clauses:
- key: source_rules
  text: '# Bracers of Archery Wondrous Item, Uncommon (Requires Attunement) While wearing these bracers,
    you have proficiency with the Longbow and Shortbow, and you gain a +2 bonus to damage rolls made with
    such weapons.'
  sha256: 2b470072b1a7e25586a9426b63cda04a38c9d1484ff644f7b03d7d63fd824048
  status: contract_specified
splat_requires_mechanics: *id001
---

# Bracers of Archery

<!-- SRD page 214 -->

Wondrous Item, Uncommon (Requires Attunement)\
While wearing these bracers, you have proficiency\
with the Longbow and Shortbow, and you gain a +2\
bonus to damage rolls made with such weapons.\

Original layout: [[../../srd-5.2.1/pages/214|page 214]].
