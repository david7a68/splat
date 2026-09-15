---
splat_schema: 1
splat_id: aaa54732-7c39-5852-949a-6d51bbaad7ed
splat_kind: spell
splat_key: dnd2024:spells/flesh_to_stone
splat_name: Flesh to Stone
splat_origin:
  key: dnd2024:reference/spell/flesh_to_stone
splat_sources:
- title: SRD 5.2.1
  locator: pp. 133
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 6
  school: transmutation
  classes:
  - druid
  - sorcerer
  - wizard
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
    description: a cockatrice feather
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: 4ae4269f6d1650016fc295ea93056a357ab30417d7648f3bd5ab536ab0cf1f84
  source_path: builtin/srd-5.2.1/spell/flesh_to_stone.md
  pages:
  - 133
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Flesh to Stone Level 6 Transmutation (Druid, Sorcerer, Wizard) Casting Time: Action Range:
    60 feet Components: V, S, M (a cockatrice feather) Duration: Concentration, up to 1 minute You attempt
    to turn one creature that you can see within range into stone. The target makes a Constitution saving
    throw. On a failed save, it has the Restrained condition for the duration. On a successful save, its
    Speed is 0 until the start of your next turn. Constructs automatically succeed on the save. A Restrained
    target makes another Constitution saving throw at the end of each of its turns. If it successfully
    saves against this spell three times, the spell ends. If it fails its saves three times, it is turned
    to stone and has the Petrified condition for the duration. The successes and failures needn’t be consecutive;
    keep track of both until the target collects three of a kind. If you maintain your Concentration on
    this spell for the entire possible duration, the target is Petrified until the condition is ended
    by Greater Restoration or similar magic.'
  sha256: b9fd5f6bc9497ff2910550adeb0df224f5e14e153cbf9ab3c6bed31856423556
  status: needs_semantic_conversion
---

# Flesh to Stone

<!-- SRD page 133 -->

Level 6 Transmutation (Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a cockatrice feather)\
Duration: Concentration, up to 1 minute\
You attempt to turn one creature that you can see\
within range into stone. The target makes a Consti-\
tution saving throw. On a failed save, it has the Re-\
strained condition for the duration. On a successful\
save, its Speed is 0 until the start of your next turn.\
Constructs automatically succeed on the save.\
A Restrained target makes another Constitution\
saving throw at the end of each of its turns. If it\
successfully saves against this spell three times,\
the spell ends. If it fails its saves three times, it is\
turned to stone and has the Petrified condition for\
the duration. The successes and failures needn’t be\
consecutive; keep track of both until the target col-\
lects three of a kind.\
If you maintain your Concentration on this spell\
for the entire possible duration, the target is Petri-\
fied until the condition is ended by Greater Resto-\
ration or similar magic.\

Original layout: [[../../srd-5.2.1/pages/133|page 133]].
