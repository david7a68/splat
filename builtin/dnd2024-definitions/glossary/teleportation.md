---
splat_schema: 1
splat_id: 7cf6c66f-fd69-576d-8ece-f0f2a6c37d83
splat_kind: rule
splat_key: dnd2024:glossary/teleportation
splat_name: Teleportation
splat_origin:
  key: dnd2024:reference/glossary/teleportation
splat_sources:
- title: SRD 5.2.1
  locator: pp. 190
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/teleportation
  parameters:
    magical: true
    instantaneous: true
    traverses_intervening_space: false
    movement_cost_default: 0
    explicit_movement_cost_override_allowed: true
    provokes_opportunity_attacks: false
    accompanying_equipment: all_worn_and_carried
    touching_creatures_accompany_by_default: false
    effect_may_allow_accompanying_creatures: true
    blocked_destination:
      blocked_if_any:
      - another_creature_present
      - solid_obstacle
      replacement: nearest_unoccupied_space
      equal_nearest_choice: teleporting_creature
    destination_sight_required: source_effect_defined
    geometry_provider: external_geometry
  implementation: dnd2024:glossary_teleportation
splat_contracts:
- id: dnd2024:glossary_teleportation
  inputs:
    subject:
      type: participant
      required: true
    destination:
      type: location
      required: true
    teleport_effect:
      type: effect
      required: true
    geometry:
      type: resolution
      required: true
    replacement_choice:
      required: false
      type: location
  outputs:
  - arrival_location
  - transported_equipment
  - movement_cost
  events:
  - teleportation.pending
  capabilities:
  - teleportation
  - geometry
  - equipment
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 3b21503f3be74754e76c0d305a517310d0d0dc81a7bb85e9d408e4aea9938d36
  source_path: builtin/srd-5.2.1/glossary/teleportation.md
  pages:
  - 190
  engine_gaps: &id001
  - dnd2024:glossary_teleportation
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 468fd5babcb030848f6ec2f4ae672c887ca208557a3397df1870504e05f3b998
splat_clauses:
- key: source_rules
  text: '# Teleportation Teleportation is a special kind of magical transportation. If you teleport, you
    disappear and reappear elsewhere instantly, without moving through the intervening space. This transportation
    doesn’t expend movement unless a rule tells you otherwise, and teleportation never provokes Opportunity
    Attacks. When you teleport, all the equipment you’re wearing and carrying teleports with you. If you’re
    touching another creature when you teleport, that creature doesn’t teleport with you unless the teleportation
    effect says otherwise. If the destination space of your teleportation is occupied by another creature
    or blocked by a solid obstacle, you instead appear in the nearest unoccupied space of your choice.
    The description of a teleportation effect tells you if you must see the teleportation’s destination.'
  sha256: d9f6414cce1930f2a9dfd3251100489180f17866ec91d0423b90055526a2e206
  status: contract_specified
splat_requires_mechanics: *id001
---

# Teleportation

<!-- SRD page 190 -->

Teleportation is a special kind of magical transpor-\
tation. If you teleport, you disappear and reappear\
elsewhere instantly, without moving through the\
intervening space. This transportation doesn’t ex-\
pend movement unless a rule tells you otherwise,\
and teleportation never provokes Opportunity\
Attacks.\
When you teleport, all the equipment you’re\
wearing and carrying teleports with you. If you’re\
touching another creature when you teleport, that\
creature doesn’t teleport with you unless the tele-\
portation effect says otherwise.\
If the destination space of your teleportation is\
occupied by another creature or blocked by a solid\
obstacle, you instead appear in the nearest unoccu-\
pied space of your choice.\
The description of a teleportation effect tells you\
if you must see the teleportation’s destination.\

Original layout: [[../../srd-5.2.1/pages/190|page 190]].
