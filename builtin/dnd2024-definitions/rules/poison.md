---
splat_schema: 1
splat_id: 213bef09-f991-5bb0-86db-d0aceb89e844
splat_kind: rule
splat_key: dnd2024:rules/poison
splat_name: Poison
splat_origin:
  key: dnd2024:reference/pages/197
splat_sources:
- title: SRD 5.2.1
  locator: pp. 197
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/poison
  parameters:
    types:
      contact:
        application: smear_on_object
        potent_until_any:
        - exposed_skin_touch
        - washed_off
        exposure: exposed_skin_touch
      ingested:
        full_effect_requires: swallow_entire_dose
        delivery:
        - food
        - liquid
        partial_dose:
          actor: gm
          choices:
          - no_effect
          - saving_throw: advantage
          - failed_save_damage_multiplier: 0.5
      inhaled:
        delivery:
        - blown_powder
        - released_gas
        area:
          shape: cube
          edge_feet: 5
        cloud_duration: immediately_dissipates_after_exposure
        holding_breath_prevents: false
      injury:
        application_action: bonus_action
        apply_to:
        - weapon
        - ammunition_piece
        - similar_object
        potent_until_any:
        - delivered_through_wound
        - washed_off
        exposure_requires:
          coated_object_damage_type:
          - piercing
          - slashing
  implementation: dnd2024:rule_poison
splat_contracts:
- id: dnd2024:rule_poison
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events: []
  capabilities:
  - poisons
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 197
  engine_gaps: &id001
  - dnd2024:rule_poison
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 0bb916b182d68c4ef417d279fafe559c733bd394bb1e70d16fe74aec3ce22c8c
  parent_name: Poison
  line_span:
    first: 9771
    count: 28
splat_clauses:
- key: source_rules
  text: '# Poison Given their insidious and deadly nature, poisons are a favorite tool among assassins
    and evil creatures. Poisons come in the following four types: Contact. Contact poison can be smeared
    on an object and remains potent until it is touched or washed off. A creature that touches contact
    poison with exposed skin suffers its effects. Ingested. A creature must swallow an entire dose of
    ingested poison to suffer its effects. The dose can be delivered in food or a liquid. You may decide
    that a partial dose has a reduced effect, such as allowing Advantage on the saving throw or dealing
    only half as much damage on a failed save. Inhaled. Poisonous powders and gases take effect when inhaled.
    Blowing the powder or releasing the gas subjects creatures in a 5-foot Cube to its effect. The resulting
    cloud dissipates immediately afterward. Holding one’s breath is ineffective against inhaled poisons,
    as they affect nasal membranes, tear ducts, and other parts of the body. Injury. Injury poison can
    be applied as a Bonus Action to a weapon, a piece of ammunition, or similar object. The poison remains
    potent until delivered through a wound or washed off. A creature that takes Piercing or Slashing damage
    from an object coated with the poison is exposed to its effects.'
  sha256: 629eab2177de7b2477468ea8789e8cd4c1ce8a749a5d963e0555d6666c24b84c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Poison

<!-- SRD page 197 -->

Given their insidious and deadly nature, poisons are\
a favorite tool among assassins and evil creatures.\
Poisons come in the following four types:\
Contact. Contact poison can be smeared on an\
object and remains potent until it is touched or\
washed off. A creature that touches contact poi-\
son with exposed skin suffers its effects.\
Ingested. A creature must swallow an entire dose\
of ingested poison to suffer its effects. The dose\
can be delivered in food or a liquid. You may de-\
cide that a partial dose has a reduced effect, such\
as allowing Advantage on the saving throw or\
dealing only half as much damage on a failed save.\
Inhaled. Poisonous powders and gases take effect\
when inhaled. Blowing the powder or releasing\
the gas subjects creatures in a 5-foot Cube to its\
effect. The resulting cloud dissipates immediately\
afterward. Holding one’s breath is ineffective\
against inhaled poisons, as they affect nasal mem-\
branes, tear ducts, and other parts of the body.\
Injury. Injury poison can be applied as a Bonus\
Action to a weapon, a piece of ammunition, or\
similar object. The poison remains potent until\
delivered through a wound or washed off. A crea-\
ture that takes Piercing or Slashing damage from\
an object coated with the poison is exposed to its\
effects.\
