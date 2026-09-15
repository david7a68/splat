---
splat_schema: 1
splat_id: 35080ca0-59f9-53a4-999d-fc4b8976915a
splat_kind: rule
splat_key: dnd2024:glossary/breaking_objects
splat_name: Breaking Objects
splat_origin:
  key: dnd2024:reference/glossary/breaking_objects
splat_sources:
- title: SRD 5.2.1
  locator: pp. 177, 178
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/breaking_objects
  parameters:
    damage_sources:
    - attacks
    - spells_that_affect_objects
    fragile_automatic_break:
      eligibility: exceedingly_fragile
      approval:
        chooser: gm
        options:
        - true
        - false
      cost_options:
      - attack_action
      - utilize_action
    suggested_ac:
      cloth: 11
      paper: 11
      rope: 11
      crystal: 13
      glass: 13
      ice: 13
      wood: 15
      stone: 17
      iron: 19
      steel: 19
      mithral: 21
      adamantine: 23
    suggested_hit_points:
      tiny:
        fragile:
          fixed: 2
          dice: 1d4
        resilient:
          fixed: 5
          dice: 2d4
      small:
        fragile:
          fixed: 3
          dice: 1d6
        resilient:
          fixed: 10
          dice: 3d6
      medium:
        fragile:
          fixed: 4
          dice: 1d8
        resilient:
          fixed: 18
          dice: 4d8
      large:
        fragile:
          fixed: 5
          dice: 1d10
        resilient:
          fixed: 27
          dice: 5d10
    statistics_selection:
      chooser: gm
      suggested_values_binding: false
      ac_type: integer
      hit_points_type: integer
    destroyed_at_hit_points: 0
    huge_or_gargantuan:
      partition_into_sizes:
      - tiny
      - small
      - medium
      - large
      hit_points_per_section: true
      whole_object_collapse_on_section_destruction:
        chooser: gm
        options:
        - true
        - false
    damage_immunities:
    - poison
    - psychic
    damage_type_effectiveness:
      chooser: gm
      scope: object_and_damage_type
      options:
      - normal
      - immune
      - resistant
      - vulnerable
      example:
        materials:
        - paper
        - cloth
        damage_type: fire
        response: vulnerable
    damage_threshold:
      allowed: true
      typical: big_objects
      rule:
        key: dnd2024:glossary/damage_threshold
    ability_scores:
      default: absent
      exception: explicit_rule_assignment
      without_scores:
        ability_checks_allowed: false
        all_saving_throws: automatic_failure
  implementation: dnd2024:glossary_breaking_objects
splat_contracts:
- id: dnd2024:glossary_breaking_objects
  inputs:
    object:
      type: object
      required: true
    attack:
      required: false
      type: resolution
    gm_choices:
      type: choice
      required: true
  outputs:
  - object_statistics
  - damage
  - destruction
  events:
  - damage.pending
  - hit_points.changed
  capabilities:
  - objects
  - checks
  - damage
  - gm_choices
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 0494e2dd7f0e08b281448246aed0244959b7bc21f922629be25d5f9b2708c619
  source_path: builtin/srd-5.2.1/glossary/breaking_objects.md
  pages:
  - 177
  - 178
  engine_gaps: &id001
  - dnd2024:glossary_breaking_objects
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 04b04515abac3043a32784b57da81b237232b0682c93ccebf69b35b9b044a5ff
splat_clauses:
- key: source_rules
  text: '# Breaking Objects Objects can be harmed by attacks and by some spells, using the rules below.
    If an object is exceedingly fragile, the GM may allow a creature to break it automatically with the
    Attack or Utilize action. Armor Class. The Object Armor Class table suggests ACs for various substances.
    Object Armor Class AC Substance AC Substance 11 Cloth, paper, rope 19 Iron, steel 13 Crystal, glass,
    ice 21 Mithral 15 Wood 23 Adamantine 17 Stone Hit Points. An object is destroyed when it has 0 Hit
    Points. The Object Hit Points table suggests Hit Points for fragile and resilient objects that are
    Large or smaller. To track Hit Points for a Huge or Gargantuan object, divide it into Large or smaller
    sections, and track each section’s Hit Points separately. The GM determines whether destroying part
    of an object causes the whole thing to collapse. Object Hit Points Size Fragile Resilient Tiny (bottle,
    lock) 2 (1d4) 5 (2d4) Small (chest, lute) 3 (1d6) 10 (3d6) Medium (barrel, chandelier) 4 (1d8) 18
    (4d8) Large (cart, dining table) 5 (1d10) 27 (5d10) Damage Types and Objects. Objects have Immunity
    to Poison and Psychic damage. The GM might decide that some damage types are more or less effective
    against an object. For example, Bludgeoning damage works well for smashing things but not for cutting.
    Paper or cloth objects might have Vulnerability to Fire damage. Damage Threshold. Big objects, such
    as castle walls, often have extra resilience represented by a damage threshold. See also “Damage Threshold.”
    No Ability Scores. An object lacks ability scores unless a rule assigns scores to the object. Without
    ability scores, an object can’t make ability checks, and it fails all saving throws.'
  sha256: bc80ba9693bd0e36553ce82d6a01aff1f3cb5d2f8f0fdee4804ad72a598d89cb
  status: contract_specified
splat_requires_mechanics: *id001
---

# Breaking Objects

<!-- SRD page 177 -->

Objects can be harmed by attacks and by some\
spells, using the rules below. If an object is exceed-\
ingly fragile, the GM may allow a creature to break\
it automatically with the Attack or Utilize action.\
Armor Class. The Object Armor Class table sug-\
gests ACs for various substances.\

<!-- SRD page 178 -->

Object Armor Class\
AC Substance AC Substance\
11 Cloth, paper, rope 19 Iron, steel\
13 Crystal, glass, ice 21 Mithral\
15 Wood 23 Adamantine\
17 Stone\
Hit Points. An object is destroyed when it has 0\
Hit Points. The Object Hit Points table suggests Hit\
Points for fragile and resilient objects that are Large\
or smaller. To track Hit Points for a Huge or Gargan-\
tuan object, divide it into Large or smaller sections,\
and track each section’s Hit Points separately. The\
GM determines whether destroying part of an ob-\
ject causes the whole thing to collapse.\
Object Hit Points\
Size Fragile Resilient\
Tiny (bottle, lock) 2 (1d4) 5 (2d4)\
Small (chest, lute) 3 (1d6) 10 (3d6)\
Medium (barrel, chandelier) 4 (1d8) 18 (4d8)\
Large (cart, dining table) 5 (1d10) 27 (5d10)\
Damage Types and Objects. Objects have Immu-\
nity to Poison and Psychic damage. The GM might\
decide that some damage types are more or less ef-\
fective against an object. For example, Bludgeoning\
damage works well for smashing things but not for\
cutting. Paper or cloth objects might have Vulnera-\
bility to Fire damage.\
Damage Threshold. Big objects, such as castle\
walls, often have extra resilience represented by a\
damage threshold. See also “Damage Threshold.”\
No Ability Scores. An object lacks ability scores\
unless a rule assigns scores to the object. Without\
ability scores, an object can’t make ability checks,\
and it fails all saving throws.\

Original layout: [[../../srd-5.2.1/pages/177|page 177]], [[../../srd-5.2.1/pages/178|page 178]].
