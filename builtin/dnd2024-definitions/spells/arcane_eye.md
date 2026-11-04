---
splat_schema: 2
splat_id: a3ca0881-5df9-56b7-866e-7a7ac7f348f9
splat_kind: spell
splat_key: dnd2024:spells/arcane_eye
splat_name: Arcane Eye
splat_origin: dnd2024:reference/spell/arcane_eye
splat_sources:
- SRD 5.2.1, pp. 110, https://www.dndbeyond.com/srd
---

# Arcane Eye


## Spell

| Field | Value |
| --- | --- |
| Level | 4 |
| School | Divination |
| Classes | Wizard |
| Casting Time | Action |
| Range | 30 feet |
| Components | V, S, M (a bit of bat fur) |
| Duration | Concentration, up to 1 hour |


## Source text

<!-- SRD page 110 -->

Level 4 Divination (Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a bit of bat fur)\
Duration: Concentration, up to 1 hour

You create an Invisible, invulnerable eye within range that hovers for the duration. You mentally receive visual information from the eye, which can see in every direction. It also has Darkvision with a range of 30 feet.

As a Bonus Action, you can move the eye up to 30 feet in any direction. A solid barrier blocks the eye’s movement, but the eye can pass through an opening as small as 1 inch in diameter.

Original layout: [[../../srd-5.2.1/pages/110|page 110]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/arcane_eye ` |
| ` implementation ` | ` dnd2024:spell_arcane_eye ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` location ` |
| ` within_range ` | ` true ` |

#### Sensor

<!-- splat:values path=splat_rule/parameters/sensor -->

| Field | Value |
| --- | --- |
| ` invisible ` | ` true ` |
| ` invulnerable ` | ` true ` |
| ` hovers ` | ` true ` |
| ` perception ` | ` visual ` |
| ` transmission ` | ` mental_to_caster ` |
| ` directions ` | ` all ` |
| ` darkvision_feet ` | ` 30 ` |
| ` duration ` | ` spell_duration ` |

#### Move

<!-- splat:values path=splat_rule/parameters/move -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` max_feet ` | ` 30 ` |
| ` direction ` | ` any ` |
| ` solid_barrier_blocks ` | ` true ` |
| ` minimum_opening_diameter_inches ` | ` 1 ` |

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
> ```

