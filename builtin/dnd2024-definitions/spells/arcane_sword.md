---
splat_schema: 1
splat_id: 7bdf42b5-f9d6-58f3-adaf-7c46a198babb
splat_kind: spell
splat_key: dnd2024:spells/arcane_sword
splat_name: Arcane Sword
splat_origin:
  key: dnd2024:reference/spell/arcane_sword
splat_sources:
- title: SRD 5.2.1
  locator: pp. 110
  url: https://www.dndbeyond.com/srd
---

# Arcane Sword

## Source text

<!-- SRD page 110 -->

Level 7 Evocation (Bard, Wizard)\
Casting Time: Action\
Range: 90 feet\
Components: V, S, M (a miniature sword worth\
250+ GP)\
Duration: Concentration, up to 1 minute\
You create a spectral sword that hovers within\
range. It lasts for the duration.\
When the sword appears, you make a melee spell\
attack against a target within 5 feet of the sword.\
On a hit, the target takes Force damage equal to\
4d12 plus your spellcasting ability modifier.\
On your later turns, you can take a Bonus Action\
to move the sword up to 30 feet to a spot you can\
see and repeat the attack against the same target or\
a different one.\

Original layout: [[../../srd-5.2.1/pages/110|page 110]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 7 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["bard", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 90 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a miniature sword worth 250+ GP ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/arcane_sword ` |
| ` implementation ` | ` dnd2024:spell_arcane_sword ` |

### Parameters

#### Creation

<!-- splat:values path=splat_rule/parameters/creation -->

| Field | Value |
| --- | --- |
| ` kind ` | ` spectral_sword ` |
| ` hover ` | ` true ` |
| ` location ` | ` within_spell_range ` |

#### Attack

<!-- splat:yaml path=splat_rule/parameters/attack -->

```yaml
event: spell.cast_completed
kind: melee_spell
target:
  kind: creature_or_object
  count: 1
  visible_to_caster: false
  distance_from_sword_feet: 5
damage:
  amount:
    dice:
      count: 4
      sides: 12
      add:
        binding: spellcasting_modifier
  type: force
```

#### Repeat

<!-- splat:values path=splat_rule/parameters/repeat -->

| Field | Value |
| --- | --- |
| ` action ` | ` bonus_action ` |
| ` timing ` | ` later_caster_turns ` |
| ` move_feet ` | ` 30 ` |
| ` destination_visible ` | ` true ` |
| ` attack ` | ` same_or_different_target ` |
