---
splat_schema: 2
splat_id: c60032dc-0ab4-5660-afd7-386b0c6fc57d
splat_kind: spell
splat_key: dnd2024:spells/resistance
splat_name: Resistance
splat_origin: dnd2024:reference/spell/resistance
splat_sources:
- SRD 5.2.1, pp. 158, https://www.dndbeyond.com/srd
---

# Resistance


## Spell

| Field | Value |
| --- | --- |
| Level | Cantrip |
| School | Abjuration |
| Classes | Cleric, Druid |
| Casting Time | Action |
| Range |  |
| Components | V, S |
| Duration | Concentration, up to 1 minute |


## Source text

<!-- SRD page 158 -->

Abjuration Cantrip (Cleric, Druid)\
Casting Time: Action\
Range: Touch\
Component: V, S\
Duration: Concentration, up to 1 minute

You touch a willing creature and choose a damage type: Acid, Bludgeoning, Cold, Fire, Lightning, Necrotic, Piercing, Poison, Radiant, Slashing, or Thunder. When the creature takes damage of the chosen type before the spell ends, the creature reduces the total damage taken by 1d4. A creature can benefit from this spell only once per turn.

Original layout: [[../../srd-5.2.1/pages/158|page 158]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/resistance ` |
| ` implementation ` | ` dnd2024:spell_resistance ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_type_choice ` | ` ["acid", "bludgeoning", "cold", "fire", "lightning", "necrotic", "piercing", "poison", "radiant", "slashing", "thunder"] ` |

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` willing ` | ` true ` |
| ` touch ` | ` true ` |

#### Reduction

<!-- splat:values path=splat_rule/parameters/reduction -->

| Field | Value |
| --- | --- |
| ` event ` | ` target.takes_chosen_damage_type ` |
| ` total_damage_reduced_by ` | ` 1d4 ` |
| ` max_benefits_per_creature_per_turn ` | ` 1 ` |
| ` duration ` | ` spell_duration ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 1
  unit: minute
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> range:
>   mode: touch
> ```

