---
splat_schema: 2
splat_id: 54d4c256-6dfc-5616-8d82-80502a91ce39
splat_kind: spell
splat_key: dnd2024:spells/beacon_of_hope
splat_name: Beacon of Hope
splat_origin: dnd2024:reference/spell/beacon_of_hope
splat_sources:
- SRD 5.2.1, pp. 112, https://www.dndbeyond.com/srd
---

# Beacon of Hope


## Spell

| Field | Value |
| --- | --- |
| Level | 3 |
| School | Abjuration |
| Classes | Cleric |
| Casting Time | Action |
| Range | 30 feet |
| Components | V, S |
| Duration | Concentration, up to 1 minute |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 112 -->

Level 3 Abjuration (Cleric)\
Casting Time: Action\
Range: 30 feet\
Components: V, S\
Duration: Concentration, up to 1 minute

Choose any number of creatures within range. For the duration, each target has Advantage on Wisdom saving throws and Death Saving Throws and regains the maximum number of Hit Points possible from any healing.

Original layout: [[../../srd-5.2.1/pages/112|page 112]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/beacon_of_hope ` |
| ` implementation ` | ` dnd2024:spell_beacon_of_hope ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` duration ` | ` spell_duration ` |
| ` advantage_on ` | ` ["wisdom_saving_throws", "death_saving_throws"] ` |

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creatures ` |
| ` count ` | ` any ` |
| ` within_range ` | ` true ` |

#### Healing

<!-- splat:values path=splat_rule/parameters/healing -->

| Field | Value |
| --- | --- |
| ` amount ` | ` maximum_possible ` |
| ` sources ` | ` all ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 1
  unit: minute
```
