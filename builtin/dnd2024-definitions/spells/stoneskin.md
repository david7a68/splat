---
splat_schema: 2
splat_id: 5c5919d9-67ca-523e-9a4d-d49534a1c746
splat_kind: spell
splat_key: dnd2024:spells/stoneskin
splat_name: Stoneskin
splat_origin: dnd2024:reference/spell/stoneskin
splat_sources:
- SRD 5.2.1, pp. 165, https://www.dndbeyond.com/srd
---

# Stoneskin


## Spell

| Field | Value |
| --- | --- |
| Level | 4 |
| School | Transmutation |
| Classes | Druid, Ranger, Sorcerer, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S, M (diamond dust worth 100+ GP, which the spell consumes) |
| Duration | Concentration, up to 1 hour |


## Source text

<!-- SRD page 165 -->

Level 4 Transmutation (Druid, Ranger, Sorcerer, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (diamond dust worth 100+ GP, which the spell consumes)\
Duration: Concentration, up to 1 hour

Until the spell ends, one willing creature you touch has Resistance to Bludgeoning, Piercing, and Slashing damage.

Original layout: [[../../srd-5.2.1/pages/165|page 165]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/stoneskin ` |
| ` implementation ` | ` dnd2024:spell_stoneskin ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` willing ` | ` true ` |
| ` touch ` | ` true ` |

#### Resistance

<!-- splat:values path=splat_rule/parameters/resistance -->

| Field | Value |
| --- | --- |
| ` types ` | ` ["bludgeoning", "piercing", "slashing"] ` |
| ` duration ` | ` spell_duration ` |
| ` magical_damage_included ` | ` true ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 1
  unit: hour
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: true
> range:
>   mode: touch
> ```

