---
splat_schema: 1
splat_id: 7ecefdcd-9575-5bde-a625-51c97bc691db
splat_kind: spell
splat_key: dnd2024:spells/heat_metal
splat_name: Heat Metal
splat_origin:
  key: dnd2024:reference/spell/heat_metal
splat_sources:
- title: SRD 5.2.1
  locator: pp. 139, 140
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: transmutation
  classes:
  - bard
  - druid
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 60
  material:
    description: a piece of iron and a flame
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/heat_metal
  implementation: dnd2024:spell_heat_metal
  parameters:
    targeting:
      kind: object
      manufactured: true
      material: metal
      visible: true
      within_range: true
    object_state:
      red_hot_glow: true
      duration: spell_duration
    damage:
      targets: creatures_physically_contacting_object
      dice: 2d8
      type: fire
      initial: on_cast
      repeat:
        economy: bonus_action
        timing: each_later_caster_turn
        requires: object_within_range
    holder_or_wearer_damaged:
      save:
        ability: constitution
        failure: drop_object_if_able
      if_not_dropped:
        disadvantage:
        - attack_rolls
        - ability_checks
        until: caster_next_turn_start
    slot_scaling:
      above: 2
      damage_per_level: 1d8
    spell_lifetime:
      concentration: true
      maximum:
        value: 1
        unit: minute
splat_contracts:
- id: dnd2024:spell_heat_metal
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    objects:
      type: object_list
      required: true
    save_dc:
      type: integer
      required: true
  outputs:
  - effects
  - damage_results
  events:
  - spell.cast_completed
  - turn.started
  - damage.applied
  - position.changed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - damage
  - spatial_effects
splat_conversion:
  status: specified
  category: spell
  source_sha256: ba5d06405140aa90eec977cfd09e1fdff2bea73df46ce22fded13e97c2b70268
  source_path: builtin/srd-5.2.1/spell/heat_metal.md
  pages:
  - 139
  - 140
  engine_gaps: &id001
  - dnd2024:spell_heat_metal
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 321988f10d2a36da16a2ae93f4acc59b2eaa465033a119f81d70ac473aeda264
splat_clauses:
- key: source_rules
  text: '# Heat Metal Level 2 Transmutation (Bard, Druid) Casting Time: Action Range: 60 feet Components:
    V, S, M (a piece of iron and a flame) Duration: Concentration, up to 1 minute Choose a manufactured
    metal object, such as a metal weapon or a suit of Heavy or Medium metal armor, that you can see within
    range. You cause the object to glow red-hot. Any creature in physical contact with the object takes
    2d8 Fire damage when you cast the spell. Until the spell ends, you can take a Bonus Action on each
    of your later turns to deal this damage again if the object is within range. If a creature is holding
    or wearing the object and takes the damage from it, the creature must succeed on a Constitution saving
    throw or drop the object if it can. If it doesn’t drop the object, it has Disadvantage on attack rolls
    and ability checks until the start of your next turn. Using a Higher-Level Spell Slot. The damage
    increases by 1d8 for each spell slot level above 2.'
  sha256: 84a3cb223f97b377d43ce03d5f35fe0b7d32c39035a4f959fea5abd399a3cbee
  status: contract_specified
splat_requires_mechanics: *id001
---

# Heat Metal

<!-- SRD page 139 -->

Level 2 Transmutation (Bard, Druid)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a piece of iron and a flame)\
Duration: Concentration, up to 1 minute\

<!-- SRD page 140 -->

Choose a manufactured metal object, such as a\
metal weapon or a suit of Heavy or Medium metal\
armor, that you can see within range. You cause\
the object to glow red-hot. Any creature in physical\
contact with the object takes 2d8 Fire damage when\
you cast the spell. Until the spell ends, you can take\
a Bonus Action on each of your later turns to deal\
this damage again if the object is within range.\
If a creature is holding or wearing the object and\
takes the damage from it, the creature must suc-\
ceed on a Constitution saving throw or drop the\
object if it can. If it doesn’t drop the object, it has\
Disadvantage on attack rolls and ability checks un-\
til the start of your next turn.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d8 for each spell slot level above 2.\

Original layout: [[../../srd-5.2.1/pages/139|page 139]], [[../../srd-5.2.1/pages/140|page 140]].
