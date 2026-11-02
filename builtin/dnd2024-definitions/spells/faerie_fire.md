---
splat_schema: 1
splat_id: 5eeefa95-08af-588f-bed1-c359739b95ce
splat_kind: spell
splat_key: dnd2024:spells/faerie_fire
splat_name: Faerie Fire
splat_origin:
  key: dnd2024:reference/spell/faerie_fire
splat_sources:
- title: SRD 5.2.1
  locator: pp. 129
  url: https://www.dndbeyond.com/srd
---

# Faerie Fire

## Source text

<!-- SRD page 129 -->

Level 1 Evocation (Bard, Druid)\
Casting Time: Action\
Range: 60 feet\
Components: V\
Duration: Concentration, up to 1 minute

Objects in a 20-foot Cube within range are outlined in blue, green, or violet light (your choice). Each creature in the Cube is also outlined if it fails a Dexterity saving throw. For the duration, objects and affected creatures shed Dim Light in a 10-foot radius and can’t benefit from the Invisible condition.

Attack rolls against an affected creature or object have Advantage if the attacker can see it.

Original layout: [[../../srd-5.2.1/pages/129|page 129]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["bard", "druid"] ` |
| ` components ` | ` ["verbal"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 60 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/faerie_fire ` |
| ` implementation ` | ` dnd2024:spell_faerie_fire ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` color_choice ` | ` ["blue", "green", "violet"] ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` cube ` |
| ` side_feet ` | ` 20 ` |

#### Objects

<!-- splat:values path=splat_rule/parameters/objects -->

| Field | Value |
| --- | --- |
| ` automatic ` | ` true ` |

#### Creatures

<!-- splat:yaml path=splat_rule/parameters/creatures -->

```yaml
save:
  ability: dexterity
  dc:
    binding: save_dc
  failure:
    outlined: true
  success:
    effect: none
```

#### Outlined

<!-- splat:values path=splat_rule/parameters/outlined -->

| Field | Value |
| --- | --- |
| ` dim_light_radius_feet ` | ` 10 ` |
| ` invisible_benefit ` | ` false ` |
| ` attack_rolls_against ` | ` advantage_if_attacker_sees_target ` |
