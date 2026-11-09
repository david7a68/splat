---
splat_schema: 2
splat_id: 59d55d3b-0a95-5744-9560-181a239903a3
splat_kind: spell
splat_key: dnd2024:spells/divine_word
splat_name: Divine Word
splat_origin: dnd2024:reference/spell/divine_word
splat_sources:
- SRD 5.2.1, pp. 125, https://www.dndbeyond.com/srd
---

# Divine Word


## Spell

| Field | Value |
| --- | --- |
| Level | 7 |
| School | Evocation |
| Classes | Cleric |
| Casting Time | Bonus Action |
| Range | 30 feet |
| Components | V |
| Duration | Instantaneous |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 125 -->

Level 7 Evocation (Cleric)\
Casting Time: Bonus Action\
Range: 30 feet\
Components: V\
Duration: Instantaneous

You utter a word imbued with power from the Upper Planes. Each creature of your choice in range makes a Charisma saving throw. On a failed save, a target that has 50 Hit Points or fewer suffers an effect based on its current Hit Points, as shown in the Divine Word Effects table. Regardless of its Hit Points, a Celestial, an Elemental, a Fey, or a Fiend target that fails its save is forced back to its plane of origin (if it isn’t there already) and can’t return to the current plane for 24 hours by any means short of a Wish spell.

**Divine Word Effects**

| Hit Points | Effect |
| --- | --- |
| 0–20 | The target dies. |
| 21–30 | The target has the Blinded, Deafened, and Stunned conditions for 1 hour. |
| 31–40 | The target has the Blinded and Deafened conditions for 10 minutes. |
| 41–50 | The target has the Deafened condition for 1 minute. |

Original layout: [[../../srd-5.2.1/pages/125|page 125]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/divine_word ` |
| ` implementation ` | ` dnd2024:spell_divine_word ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creatures ` |
| ` chosen ` | ` true ` |
| ` within_range ` | ` true ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` charisma ` |
| ` success ` | ` no_effect ` |

#### Failure hp branches

<!-- splat:table
path: splat_rule/parameters/failure_hp_branches
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Hp min:
    path: hp_min
    type: integer
  Hp max:
    path: hp_max
    type: integer
  Effect:
    path: effect
    type: symbol
    optional: true
  Conditions:
    path: conditions
    type: yaml
    optional: true
  Duration minutes:
    path: duration_minutes
    type: integer
    optional: true
-->

| Key | Hp min | Hp max | Effect | Conditions | Duration minutes |
| --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 0 ` | ` 20 ` | ` Death ` |  |  |
| ` row_2 ` | ` 21 ` | ` 30 ` |  | ` ["blinded", "deafened", "stunned"] ` | ` 60 ` |
| ` row_3 ` | ` 31 ` | ` 40 ` |  | ` ["blinded", "deafened"] ` | ` 10 ` |
| ` row_4 ` | ` 41 ` | ` 50 ` |  | ` ["deafened"] ` | ` 1 ` |

#### Additional failure

<!-- splat:values path=splat_rule/parameters/additional_failure -->

| Field | Value |
| --- | --- |
| ` types ` | ` ["celestial", "elemental", "fey", "fiend"] ` |
| ` hp_independent ` | ` true ` |
| ` destination ` | ` origin_plane ` |
| ` only_if_elsewhere ` | ` true ` |
| ` return_to_departed_plane_forbidden_hours ` | ` 24 ` |
| ` bypass ` | ` wish ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  kind: instantaneous
```
