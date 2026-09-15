---
splat_schema: 1
splat_id: 4a838293-61fd-5c3a-a04a-156e714967c3
splat_kind: spell
splat_key: dnd2024:spells/produce_flame
splat_name: Produce Flame
splat_origin:
  key: dnd2024:reference/spell/produce_flame
splat_sources:
- title: SRD 5.2.1
  locator: pp. 156
  url: https://www.dndbeyond.com/srd
---

# Produce Flame

## Source text

<!-- SRD page 156 -->

Conjuration Cantrip (Druid)\
Casting Time: Bonus Action\
Range: Self\
Components: V, S\
Duration: 10 minutes\
A flickering flame appears in your hand and re-\
mains there for the duration. While there, the flame\
emits no heat and ignites nothing, and it sheds\
Bright Light in a 20-foot radius and Dim Light for an\
additional 20 feet. The spell ends if you cast it again.\
Until the spell ends, you can take a Magic action to\
hurl fire at a creature or an object within 60 feet of\
you. Make a ranged spell attack. On a hit, the target\
takes 1d8 Fire damage.\
Cantrip Upgrade. The damage increases by 1d8\
when you reach levels 5 (2d8), 11 (3d8), and 17\
(4d8).\

Original layout: [[../../srd-5.2.1/pages/156|page 156]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 0 ` |
| ` school ` | ` conjuration ` |
| ` classes ` | ` ["druid"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 10 minutes ` |
| ` casting_time ` | ` Bonus Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` self ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/produce_flame ` |
| ` implementation ` | ` dnd2024:spell_produce_flame ` |

### Parameters

#### Flame

<!-- splat:values path=splat_rule/parameters/flame -->

| Field | Value |
| --- | --- |
| ` location ` | ` caster_hand ` |
| ` duration ` | ` spell_duration ` |
| ` emits_heat ` | ` false ` |
| ` ignites_objects ` | ` false ` |
| ` bright_radius_feet ` | ` 20 ` |
| ` additional_dim_feet ` | ` 20 ` |
| ` end_on_recast ` | ` true ` |

#### Hurl

<!-- splat:yaml path=splat_rule/parameters/hurl -->

```yaml
cost: magic_action
available_until_spell_ends: true
target:
  kind: creature_or_object
  maximum_distance_feet: 60
attack: ranged_spell
hit:
  dice:
    count:
      by_caster_level:
      - minimum: 1
        count: 1
      - minimum: 5
        count: 2
      - minimum: 11
        count: 3
      - minimum: 17
        count: 4
    sides: 8
    add: 0
  type: fire
ends_spell: false
```
