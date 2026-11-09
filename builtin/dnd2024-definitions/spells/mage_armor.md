---
splat_schema: 2
splat_id: 397d1e99-e0d9-5cd4-bb11-7a29716e088d
splat_kind: spell
splat_key: dnd2024:spells/mage_armor
splat_name: Mage Armor
splat_origin: dnd2024:reference/spell/mage_armor
splat_sources:
- SRD 5.2.1, pp. 145, https://www.dndbeyond.com/srd
---

# Mage Armor


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Abjuration |
| Classes | Sorcerer, Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S, M (a piece of cured leather) |
| Duration | 8 hours |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 145 -->

Level 1 Abjuration (Sorcerer, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a piece of cured leather)\
Duration: 8 hours

You touch a willing creature who isn’t wearing armor. Until the spell ends, the target’s base AC becomes 13 plus its Dexterity modifier. The spell ends early if the target dons armor.

Original layout: [[../../srd-5.2.1/pages/145|page 145]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/mage_armor ` |
| ` implementation ` | ` dnd2024:spell_mage_armor ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` maximum ` | ` 1 ` |
| ` within_spell_range ` | ` true ` |
| ` touched ` | ` true ` |
| ` willing ` | ` true ` |
| ` wearing_armor ` | ` false ` |

#### Base ac

<!-- splat:values path=splat_rule/parameters/base_ac -->

| Field | Value |
| --- | --- |
| ` base ` | ` 13 ` |
| ` add ` | ` target_dexterity_modifier ` |
| ` duration ` | ` spell_duration ` |

#### End when

<!-- splat:values path=splat_rule/parameters/end_when -->

| Field | Value |
| --- | --- |
| ` target_dons_armor ` | ` true ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: touch
> ```

