---
splat_schema: 1
splat_id: 1642deb2-80be-57eb-b4e0-0867bc724528
splat_kind: spell
splat_key: dnd2024:spells/power_word_kill
splat_name: Power Word Kill
splat_origin:
  key: dnd2024:reference/spell/power_word_kill
splat_sources:
- title: SRD 5.2.1
  locator: pp. 154
  url: https://www.dndbeyond.com/srd
---

# Power Word Kill

## Source text

<!-- SRD page 154 -->

Level 9 Enchantment (Bard, Sorcerer, Warlock,\
Wizard)\
Casting Time: Action\
Range: 60 feet\
Component: V\
Duration: Instantaneous\
You compel one creature you can see within range\
to die. If the target has 100 Hit Points or fewer, it\
dies. Otherwise, it takes 12d12 Psychic damage.\

Original layout: [[../../srd-5.2.1/pages/154|page 154]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 9 ` |
| ` school ` | ` enchantment ` |
| ` classes ` | ` ["bard", "sorcerer", "warlock", "wizard"] ` |
| ` components ` | ` ["verbal"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
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
| ` mechanic ` | ` dnd2024:spells/power_word_kill ` |
| ` implementation ` | ` dnd2024:spell_power_word_kill ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` instantaneous ` | ` true ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` visible ` | ` true ` |
| ` range_feet ` | ` 60 ` |

#### Branches

<!-- splat:yaml path=splat_rule/parameters/branches -->

```yaml
- when:
    stat: current_hit_points
    comparison: lte
    value: 100
  effect: die
- when:
    stat: current_hit_points
    comparison: gt
    value: 100
  damage:
    dice: 12d12
    type: psychic
```
