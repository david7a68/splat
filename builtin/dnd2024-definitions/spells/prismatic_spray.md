---
splat_schema: 1
splat_id: 815fe32c-d12a-540a-acde-9d65b2337972
splat_kind: spell
splat_key: dnd2024:spells/prismatic_spray
splat_name: Prismatic Spray
splat_origin:
  key: dnd2024:reference/spell/prismatic_spray
splat_sources:
- title: SRD 5.2.1
  locator: pp. 154, 155
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 7
  school: evocation
  classes:
  - bard
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    mode: self
splat_conversion:
  status: partial
  category: spell
  source_sha256: f333bfebbc42fc455131320843292c8c5c37770cefcf704675227eff912ef907
  source_path: builtin/srd-5.2.1/spell/prismatic_spray.md
  pages:
  - 154
  - 155
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Prismatic Spray Level 7 Evocation (Bard, Sorcerer, Wizard) Casting Time: Action Range: Self
    Components: V, S Duration: Instantaneous Eight rays of light flash from you in a 60-foot Cone. Each
    creature in the Cone makes a Dexterity saving throw. For each target, roll 1d8 to determine which
    color ray affects it, consulting the Prismatic Rays table. Prismatic Rays 1d8 Ray 1 Red. Failed Save:
    12d6 Fire damage. Successful Save: Half as much damage. 2 Orange. Failed Save: 12d6 Acid damage. Successful
    Save: Half as much damage. 3 Yellow. Failed Save: 12d6 Lightning damage. Successful Save: Half as
    much damage. 4 Green. Failed Save: 12d6 Poison damage. Successful Save: Half as much damage. 1d8 Ray
    5 Blue. Failed Save: 12d6 Cold damage. Successful Save: Half as much damage. 6 Indigo. Failed Save:
    The target has the Restrained condition and makes a Constitution saving throw at the end of each of
    its turns. If it successfully saves three times, the condition ends. If it fails three times, it has
    the Petrified condition until it is freed by an effect like the Greater Restoration spell. The successes
    and failures needn’t be consecutive; keep track of both until the target collects three of a kind.
    7 Violet. Failed Save: The target has the Blinded condition and makes a Wisdom saving throw at the
    start of your next turn. On a successful save, the condition ends. On a failed save, the condition
    ends, and the creature teleports to another plane of existence (GM’s choice). 8 Special. The target
    is struck by two rays. Roll twice, rerolling any 8.'
  sha256: 493d582a50e16f7327f862621a46fca299f0284b5e750e4796a15cc55fa2334d
  status: needs_semantic_conversion
---

# Prismatic Spray

<!-- SRD page 154 -->

Level 7 Evocation (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: Instantaneous\
Eight rays of light flash from you in a 60-foot Cone.\
Each creature in the Cone makes a Dexterity saving\
throw. For each target, roll 1d8 to determine which\
color ray affects it, consulting the Prismatic Rays\
table.\
Prismatic Rays\
1d8 Ray\
1 Red. Failed Save: 12d6 Fire damage. Successful\
Save: Half as much damage.\
2 Orange. Failed Save: 12d6 Acid damage. Successful\
Save: Half as much damage.\
3 Yellow. Failed Save: 12d6 Lightning damage. Suc-\
cessful Save: Half as much damage.\
4 Green. Failed Save: 12d6 Poison damage. Success-\
ful Save: Half as much damage.\

<!-- SRD page 155 -->

1d8 Ray\
5 Blue. Failed Save: 12d6 Cold damage. Successful\
Save: Half as much damage.\
6 Indigo. Failed Save: The target has the Restrained\
condition and makes a Constitution saving throw\
at the end of each of its turns. If it successfully\
saves three times, the condition ends. If it fails\
three times, it has the Petrified condition until it\
is freed by an effect like the Greater Restoration\
spell. The successes and failures needn’t be con-\
secutive; keep track of both until the target col-\
lects three of a kind.\
7 Violet. Failed Save: The target has the Blinded\
condition and makes a Wisdom saving throw at\
the start of your next turn. On a successful save,\
the condition ends. On a failed save, the condi-\
tion ends, and the creature teleports to another\
plane of existence (GM’s choice).\
8 Special. The target is struck by two rays. Roll\
twice, rerolling any 8.\

Original layout: [[../../srd-5.2.1/pages/154|page 154]], [[../../srd-5.2.1/pages/155|page 155]].
