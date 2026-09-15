---
splat_schema: 1
splat_id: 59d55d3b-0a95-5744-9560-181a239903a3
splat_kind: spell
splat_key: dnd2024:spells/divine_word
splat_name: Divine Word
splat_origin:
  key: dnd2024:reference/spell/divine_word
splat_sources:
- title: SRD 5.2.1
  locator: pp. 125
  url: https://www.dndbeyond.com/srd
---

# Divine Word

## Source text

<!-- SRD page 125 -->

Level 7 Evocation (Cleric)\
Casting Time: Bonus Action\
Range: 30 feet\
Components: V\
Duration: Instantaneous\
You utter a word imbued with power from the Up-\
per Planes. Each creature of your choice in range\
makes a Charisma saving throw. On a failed save,\
a target that has 50 Hit Points or fewer suffers an\
effect based on its current Hit Points, as shown in\
the Divine Word Effects table. Regardless of its Hit\
Points, a Celestial, an Elemental, a Fey, or a Fiend\
target that fails its save is forced back to its plane of\
origin (if it isn’t there already) and can’t return to\
the current plane for 24 hours by any means short\
of a Wish spell.\
Divine Word Effects\
Hit Points Effect\
0–20 The target dies.\
21–30 The target has the Blinded, Deafened, and\
Stunned conditions for 1 hour.\
31–40 The target has the Blinded and Deafened\
conditions for 10 minutes.\
41–50 The target has the Deafened condition for\
1 minute.\

Original layout: [[../../srd-5.2.1/pages/125|page 125]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 7 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["cleric"] ` |
| ` components ` | ` ["verbal"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Bonus Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 30 ` |

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

<!-- splat:rows path=splat_rule/parameters/failure_hp_branches -->

| Row | hp_min | hp_max | effect | conditions | duration_minutes |
| --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` 0 ` | ` 20 ` | ` death ` |  |  |
| ` 1 ` | ` 21 ` | ` 30 ` |  | ` ["blinded", "deafened", "stunned"] ` | ` 60 ` |
| ` 2 ` | ` 31 ` | ` 40 ` |  | ` ["blinded", "deafened"] ` | ` 10 ` |
| ` 3 ` | ` 41 ` | ` 50 ` |  | ` ["deafened"] ` | ` 1 ` |

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
