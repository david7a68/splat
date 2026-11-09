---
splat_schema: 2
splat_id: 711a82f5-bc93-52b3-8e2a-79c734bc7eb4
splat_kind: spell
splat_key: dnd2024:spells/spare_the_dying
splat_name: Spare the Dying
splat_origin: dnd2024:reference/spell/spare_the_dying
splat_sources:
- SRD 5.2.1, pp. 163, https://www.dndbeyond.com/srd
---

# Spare the Dying


## Spell

| Field | Value |
| --- | --- |
| Level | Cantrip |
| School | Necromancy |
| Classes | Cleric, Druid |
| Casting Time | Action |
| Range | 15 feet |
| Components | V, S |
| Duration | Instantaneous |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 163 -->

Necromancy Cantrip (Cleric, Druid)\
Casting Time: Action\
Range: 15 feet\
Components: V, S\
Duration: Instantaneous

Choose a creature within range that has 0 Hit Points and isn’t dead. The creature becomes Stable.

Cantrip Upgrade. The range doubles when you reach levels 5 (30 feet), 11 (60 feet), and 17 (120 feet).

Original layout: [[../../srd-5.2.1/pages/163|page 163]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/spare_the_dying ` |
| ` implementation ` | ` dnd2024:spell_spare_the_dying ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` health_transition ` | ` stabilize ` |

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` hp ` | ` 0 ` |
| ` dead ` | ` false ` |

##### Maximum range feet

###### By caster level

<!-- splat:table
path: splat_rule/parameters/targets/maximum_range_feet/by_caster_level
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Minimum:
    path: minimum
    type: integer
  Value:
    path: value
    type: integer
-->

| Key | Minimum | Value |
| --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` 15 ` |
| ` row_2 ` | ` 5 ` | ` 30 ` |
| ` row_3 ` | ` 11 ` | ` 60 ` |
| ` row_4 ` | ` 17 ` | ` 120 ` |
