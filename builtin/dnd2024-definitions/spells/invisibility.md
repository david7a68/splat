---
splat_schema: 2
splat_id: 1d9f348b-1d10-5840-b28b-6160733877bb
splat_kind: spell
splat_key: dnd2024:spells/invisibility
splat_name: Invisibility
splat_origin: dnd2024:reference/spell/invisibility
splat_sources:
- SRD 5.2.1, pp. 143, https://www.dndbeyond.com/srd
---

# Invisibility


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Illusion |
| Classes | Bard, Sorcerer, Warlock, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S, M (an eyelash in gum arabic) |
| Duration | Concentration, up to 1 hour |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 143 -->

Level 2 Illusion (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (an eyelash in gum arabic)\
Duration: Concentration, up to 1 hour

A creature you touch has the Invisible condition until the spell ends. The spell ends early immediately after the target makes an attack roll, deals damage, or casts a spell.

Using a Higher-Level Spell Slot. You can target one additional creature for each spell slot level above 2.

Original layout: [[../../srd-5.2.1/pages/143|page 143]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/invisibility ` |
| ` implementation ` | ` dnd2024:spell_invisibility ` |

### Parameters

#### Targeting

<!-- splat:yaml path=splat_rule/parameters/targeting -->

```yaml
kind: creatures
touch: true
count:
  base: 1
  per_slot_above: 1
  slot_base: 2
```

#### Condition

<!-- splat:values path=splat_rule/parameters/condition -->

| Field | Value |
| --- | --- |
| ` name ` | ` invisible ` |
| ` duration ` | ` spell_duration ` |

#### Early end

<!-- splat:values path=splat_rule/parameters/early_end -->

| Field | Value |
| --- | --- |
| ` timing ` | ` immediately_after ` |
| ` target_events ` | ` ["attack_roll_made", "damage_dealt", "spell_cast"] ` |
| ` scope ` | ` spell ` |

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
>   consumed: false
> range:
>   mode: touch
> ```

