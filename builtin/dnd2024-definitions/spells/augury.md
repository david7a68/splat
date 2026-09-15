---
splat_schema: 1
splat_id: dafa5e63-f21b-5e6c-b07f-be909cd51747
splat_kind: spell
splat_key: dnd2024:spells/augury
splat_name: Augury
splat_origin:
  key: dnd2024:reference/spell/augury
splat_sources:
- title: SRD 5.2.1
  locator: pp. 111
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: divination
  classes:
  - cleric
  - druid
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: 1 minute or Ritual
  ritual: true
  range:
    mode: self
  material:
    description: specially marked sticks, bones, cards, or other divinatory tokens worth 25+ GP
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: 938bb2bb8cdfb04c38e02535051cc815542e2ae1976fb898814fcbd8123ae081
  source_path: builtin/srd-5.2.1/spell/augury.md
  pages:
  - 111
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Augury Level 2 Divination (Cleric, Druid, Wizard) Casting Time: 1 minute or Ritual Range: Self
    Components: V, S, M (specially marked sticks, bones, cards, or other divinatory tokens worth 25+ GP)
    Duration: Instantaneous You receive an omen from an otherworldly entity about the results of a course
    of action that you plan to take within the next 30 minutes. The GM chooses the omen from the Omens
    table. Omens Omen For Results That Will Be … Weal Good Woe Bad Weal and woe Good and bad Indifference
    Neither good nor bad The spell doesn’t account for circumstances, such as other spells, that might
    change the results. If you cast the spell more than once before finishing a Long Rest, there is a
    cumulative 25 percent chance for each casting after the first that you get no answer.'
  sha256: ae33cef49f38a07d4d3b05f7518f9cb9efdb7cdf96c619367930d0f88d31f4db
  status: needs_semantic_conversion
---

# Augury

<!-- SRD page 111 -->

Level 2 Divination (Cleric, Druid, Wizard)\
Casting Time: 1 minute or Ritual\
Range: Self\
Components: V, S, M (specially marked sticks, bones,\
cards, or other divinatory tokens worth 25+ GP)\
Duration: Instantaneous\
You receive an omen from an otherworldly entity\
about the results of a course of action that you plan\
to take within the next 30 minutes. The GM chooses\
the omen from the Omens table.\
Omens\
Omen For Results That Will Be …\
Weal Good\
Woe Bad\
Weal and woe Good and bad\
Indifference Neither good nor bad\
The spell doesn’t account for circumstances, such as\
other spells, that might change the results.\
If you cast the spell more than once before finish-\
ing a Long Rest, there is a cumulative 25 percent\
chance for each casting after the first that you get\
no answer.\

Original layout: [[../../srd-5.2.1/pages/111|page 111]].
