---
splat_schema: 1
splat_id: ac9606ad-e79a-552d-9f6e-f3a401571dfe
splat_kind: spell
splat_key: dnd2024:spells/detect_magic
splat_name: Detect Magic
splat_origin:
  key: dnd2024:reference/spell/detect_magic
splat_sources:
- title: SRD 5.2.1
  locator: pp. 123
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: divination
  classes:
  - bard
  - cleric
  - druid
  - paladin
  - ranger
  - sorcerer
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Action or Ritual
  ritual: true
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/detect_magic
  implementation: dnd2024:spell_detect_magic
  parameters:
    targeting:
      kind: self
    sense:
      radius_feet: 30
      subject: magical_effect_presence
      duration: spell_duration
    reveal:
      requires: magic_sensed
      economy: magic_action
      aura_on: visible_creatures_and_objects_bearing_magic
      spell_created_effect: reveal_school
    blocked_by:
    - material: stone
      thickness_inches: 12
    - material: dirt
      thickness_inches: 12
    - material: wood
      thickness_inches: 12
    - material: metal
      thickness_inches: 1
    - material: lead
      thickness: thin_sheet
    spell_lifetime:
      concentration: true
      maximum:
        value: 10
        unit: minute
splat_contracts:
- id: dnd2024:spell_detect_magic
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
  outputs:
  - effects
  events:
  - spell.cast_completed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
splat_conversion:
  status: specified
  category: spell
  source_sha256: 2eb881985474949719017d22c1e0544213661369a908ed604a2f1ac0ea314983
  source_path: builtin/srd-5.2.1/spell/detect_magic.md
  pages:
  - 123
  engine_gaps: &id001
  - dnd2024:spell_detect_magic
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 3b21e184b694fc38326ccf44649e434ce522e9f9f9d6eddb89b4a56eb6dd6e23
splat_clauses:
- key: source_rules
  text: '# Detect Magic Level 1 Divination (Bard, Cleric, Druid, Paladin, Ranger, Sorcerer, Warlock, Wizard)
    Casting Time: Action or Ritual Range: Self Components: V, S Duration: Concentration, up to 10 minutes
    For the duration, you sense the presence of magical effects within 30 feet of yourself. If you sense
    such effects, you can take the Magic action to see a faint aura around any visible creature or object
    in the area that bears the magic, and if an effect was created by a spell, you learn the spell’s school
    of magic. The spell is blocked by 1 foot of stone, dirt, or wood; 1 inch of metal; or a thin sheet
    of lead.'
  sha256: bd2534410cf1b08794056e5161a2f4c6129e00ebee07d843dabd5c094f5082e6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Detect Magic

<!-- SRD page 123 -->

Level 1 Divination (Bard, Cleric, Druid, Paladin,\
Ranger, Sorcerer, Warlock, Wizard)\
Casting Time: Action or Ritual\
Range: Self\
Components: V, S\
Duration: Concentration, up to 10 minutes\
For the duration, you sense the presence of magical\
effects within 30 feet of yourself. If you sense such\
effects, you can take the Magic action to see a faint\
aura around any visible creature or object in the\
area that bears the magic, and if an effect was cre-\
ated by a spell, you learn the spell’s school of magic.\
The spell is blocked by 1 foot of stone, dirt, or\
wood; 1 inch of metal; or a thin sheet of lead.\

Original layout: [[../../srd-5.2.1/pages/123|page 123]].
