---
splat_schema: 1
splat_id: 929037b2-6db0-5d5f-ba5f-685ca8d83dd9
splat_kind: spell
splat_key: dnd2024:spells/dream
splat_name: Dream
splat_origin:
  key: dnd2024:reference/spell/dream
splat_sources:
- title: SRD 5.2.1
  locator: pp. 126
  url: https://www.dndbeyond.com/srd
---

# Dream

## Source text

<!-- SRD page 126 -->

Level 5 Illusion (Bard, Warlock, Wizard)\
Casting Time: 1 minute\
Range: Special\
Components: V, S, M (a handful of sand)\
Duration: 8 hours

You target a creature you know on the same plane of existence. You or a willing creature you touch enters a trance state to act as a dream messenger. While in the trance, the messenger is Incapacitated and has a Speed of 0.

If the target is asleep, the messenger appears in the target’s dreams and can converse with the target as long as it remains asleep, through the spell’s duration. The messenger can also shape the dream’s environment, creating landscapes, objects, and other images. The messenger can emerge from the trance at any time, ending the spell. The target recalls the dream perfectly upon waking.

If the target is awake when you cast the spell, the messenger knows it and can either end the trance (and the spell) or wait for the target to sleep, at which point the messenger enters its dreams.

You can make the messenger terrifying to the target. If you do so, the messenger can deliver a message of no more than ten words, and then the target makes a Wisdom saving throw. On a failed save, the target gains no benefit from its rest, and it takes 3d6 Psychic damage when it wakes up.

Original layout: [[../../srd-5.2.1/pages/126|page 126]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 5 ` |
| ` school ` | ` illusion ` |
| ` classes ` | ` ["bard", "warlock", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 8 hours ` |
| ` casting_time ` | ` 1 minute ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` special ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a handful of sand ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/dream ` |
| ` implementation ` | ` dnd2024:spell_dream ` |

### Parameters

#### Targets

<!-- splat:yaml path=splat_rule/parameters/targets -->

```yaml
dream_target:
  kind: creature
  known_to_caster: true
  same_plane_as_caster: true
messenger:
  choices:
  - caster
  - willing_touched_creature
```

#### Trance

<!-- splat:values path=splat_rule/parameters/trance -->

| Field | Value |
| --- | --- |
| ` condition ` | ` incapacitated ` |
| ` speed ` | ` 0 ` |
| ` end_voluntarily_any_time ` | ` true ` |
| ` on_end ` | ` end_spell ` |

#### Dream

<!-- splat:yaml path=splat_rule/parameters/dream -->

```yaml
requires: target_asleep
conversation_until:
- target_wakes
- spell_ends
messenger_can_shape:
- landscapes
- objects
- images
target_recall: perfect
target_initially_awake:
  messenger_knows: true
  choices:
  - end_trance
  - wait_for_sleep
```

#### Terrifying option

<!-- splat:yaml path=splat_rule/parameters/terrifying_option -->

```yaml
message_word_limit: 10
save_after_message:
  ability: wisdom
  dc:
    binding: save_dc
  failure:
    rest_benefits: false
    damage_when_target_wakes:
      dice:
        count: 3
        sides: 6
        add: 0
      type: psychic
  success:
    effect: none
```
