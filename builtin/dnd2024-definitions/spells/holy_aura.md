---
splat_schema: 2
splat_id: 94b66205-d1c9-5a86-8d01-2c6c7c49c9e9
splat_kind: spell
splat_key: dnd2024:spells/holy_aura
splat_name: Holy Aura
splat_origin: dnd2024:reference/spell/holy_aura
splat_sources:
- SRD 5.2.1, pp. 141, https://www.dndbeyond.com/srd
---

# Holy Aura


## Spell

| Field | Value |
| --- | --- |
| Level | 8 |
| School | Abjuration |
| Classes | Cleric |
| Casting Time | Action |
| Range |  |
| Components | V, S, M (a reliquary worth 1,000+ GP) |
| Duration | Concentration, up to 1 minute |


## Source text

<!-- SRD page 141 -->

Level 8 Abjuration (Cleric)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a reliquary worth 1,000+ GP)\
Duration: Concentration, up to 1 minute

For the duration, you emit an aura in a 30-foot Emanation. While in the aura, creatures of your choice have Advantage on all saving throws, and other creatures have Disadvantage on attack rolls against them. In addition, when a Fiend or an Undead hits an affected creature with a melee attack roll, the attacker must succeed on a Constitution saving throw or have the Blinded condition until the end of its next turn.

Original layout: [[../../srd-5.2.1/pages/141|page 141]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/holy_aura ` |
| ` implementation ` | ` dnd2024:spell_holy_aura ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` duration ` | ` spell_duration ` |

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` chosen_creatures_in_aura ` |
| ` shape ` | ` emanation ` |
| ` radius_feet ` | ` 30 ` |
| ` origin ` | ` caster ` |

#### Benefits while inside

<!-- splat:values path=splat_rule/parameters/benefits_while_inside -->

| Field | Value |
| --- | --- |
| ` saving_throws ` | ` advantage ` |
| ` attacks_by_other_creatures_against_affected ` | ` disadvantage ` |

#### Retaliation

<!-- splat:yaml path=splat_rule/parameters/retaliation -->

```yaml
event: melee_attack_roll_hits_affected_creature
attacker_types:
- fiend
- undead
save:
  ability: constitution
  failure:
    condition: blinded
    until: attacker_next_turn_end
```

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
> material:
>   consumed: false
> range:
>   mode: self
> ```

