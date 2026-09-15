---
splat_schema: 1
splat_id: c0b0eb2c-04e7-5e92-932b-2ad461c3a302
splat_kind: spell
splat_key: dnd2024:spells/spider_climb
splat_name: Spider Climb
splat_origin:
  key: dnd2024:reference/spell/spider_climb
splat_sources:
- title: SRD 5.2.1
  locator: pp. 164
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: transmutation
  classes:
  - sorcerer
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 hour
  casting_time: Action
  ritual: false
  range:
    mode: touch
  material:
    description: a drop of bitumen and a spider
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/spider_climb
  implementation: dnd2024:spell_spider_climb
  parameters:
    targets:
      kind: creature
      maximum:
        op: add
        args:
        - 1
        - op: multiply
          args:
          - 1
          - op: subtract
            args:
            - binding: cast_level
            - 2
      within_spell_range: true
      willing: true
      touched: true
    movement:
      duration: spell_duration
      surfaces:
      - vertical_up
      - vertical_down
      - vertical_across
      - ceilings
      hands_remain_free: true
      climb_speed: target_speed
splat_contracts:
- id: dnd2024:spell_spider_climb
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    targets:
      type: participant_list
      required: true
    locations:
      required: false
      type: location_list
    campaign:
      required: false
      type: campaign
    choices:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - spell.cast_completed
  - movement.completed
  capabilities:
  - spellcasting
  - effects
  - spatial_relationships
splat_conversion:
  status: specified
  category: spell
  source_sha256: a82eeeb8842ec3e1777bfb36e7a8ff4aada77c4b4c95f3f4cc68416ed5b353b1
  source_path: builtin/srd-5.2.1/spell/spider_climb.md
  pages:
  - 164
  engine_gaps: &id001
  - dnd2024:spell_spider_climb
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: bbd69159e1ffa4f2290ec94040ba818d781a3aeb262bc87f002a4104da99d299
splat_clauses:
- key: source_rules
  text: '# Spider Climb Level 2 Transmutation (Sorcerer, Warlock, Wizard) Casting Time: Action Range:
    Touch Components: V, S, M (a drop of bitumen and a spider) Duration: Concentration, up to 1 hour Until
    the spell ends, one willing creature you touch gains the ability to move up, down, and across vertical
    surfaces and along ceilings, while leaving its hands free. The target also gains a Climb Speed equal
    to its Speed. Using a Higher-Level Spell Slot. You can target one additional creature for each spell
    slot level above 2.'
  sha256: 7efd4a7634d93c7b73ccc291d215940d666020bb2927009bebc15dcc52da4ace
  status: contract_specified
splat_requires_mechanics: *id001
---

# Spider Climb

<!-- SRD page 164 -->

Level 2 Transmutation (Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a drop of bitumen and a spider)\
Duration: Concentration, up to 1 hour\
Until the spell ends, one willing creature you touch\
gains the ability to move up, down, and across\
vertical surfaces and along ceilings, while leaving\
its hands free. The target also gains a Climb Speed\
equal to its Speed.\
Using a Higher-Level Spell Slot. You can target\
one additional creature for each spell slot level\
above 2.\

Original layout: [[../../srd-5.2.1/pages/164|page 164]].
