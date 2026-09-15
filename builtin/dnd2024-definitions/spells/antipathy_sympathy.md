---
splat_schema: 1
splat_id: eb5016a9-6e24-5df0-8b55-1fde1a9b0d32
splat_kind: spell
splat_key: dnd2024:spells/antipathy_sympathy
splat_name: Antipathy/Sympathy
splat_origin:
  key: dnd2024:reference/spell/antipathy_sympathy
splat_sources:
- title: SRD 5.2.1
  locator: pp. 109, 110
  url: https://www.dndbeyond.com/srd
---

# Antipathy/Sympathy

## Source text

<!-- SRD page 109 -->

Level 8 Enchantment (Bard, Druid, Wizard)\
Casting Time: 1 hour\
Range: 60 feet\
Components: V, S, M (a mix of vinegar and honey)\
Duration: 10 days\
As you cast the spell, choose whether it creates\
antipathy or sympathy, and target one creature or\
object that is Huge or smaller. Then specify a kind of\
creature, such as red dragons, goblins, or vampires.\
A creature of the chosen kind makes a Wisdom\
saving throw when it comes within 120 feet of the\
target. Your choice of antipathy or sympathy deter-\
mines what happens to a creature when it fails that\
save:\
Antipathy. The creature has the Frightened condi-\
tion. The Frightened creature must use its move-\
ment on its turns to get as far away as possible\
from the target, moving by the safest route.\
Sympathy. The creature has the Charmed condi-\
tion. The Charmed creature must use its move-\
ment on its turns to get as close as possible to the\
target, moving by the safest route. If the creature\
is within 5 feet of the target, the creature can’t\
willingly move away. If the target damages the\
Charmed creature, that creature can make a\

<!-- SRD page 110 -->

Wisdom saving throw to end the effect, as de-\
scribed below.\
Ending the Effect. If the Frightened or Charmed\
creature ends its turn more than 120 feet away\
from the target, the creature makes a Wisdom sav-\
ing throw. On a successful save, the creature is no\
longer affected by the target. A creature that suc-\
cessfully saves against this effect is immune to it for\
1 minute, after which it can be affected again.\

Original layout: [[../../srd-5.2.1/pages/109|page 109]], [[../../srd-5.2.1/pages/110|page 110]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 8 ` |
| ` school ` | ` enchantment ` |
| ` classes ` | ` ["bard", "druid", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 10 days ` |
| ` casting_time ` | ` 1 hour ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 60 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a mix of vinegar and honey ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/antipathy_sympathy ` |
| ` implementation ` | ` dnd2024:spell_antipathy_sympathy ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature_or_object ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` false ` |
| ` maximum_size ` | ` huge ` |

#### Creature kind choice

<!-- splat:values path=splat_rule/parameters/creature_kind_choice -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` caster ` |
| ` selection ` | ` creature_kind ` |

#### Aura

<!-- splat:yaml path=splat_rule/parameters/aura -->

```yaml
radius_feet: 120
event: area.enter
save:
  ability: wisdom
  dc:
    binding: save_dc
  failure:
    apply_chosen_mode: true
  success:
    effect: none
```

#### Modes

<!-- splat:yaml path=splat_rule/parameters/modes -->

```yaml
antipathy:
  condition: frightened
  movement:
    event: turn.current
    direction: away_from_target
    maximize_distance: true
    route: safest
sympathy:
  condition: charmed
  movement:
    event: turn.current
    direction: toward_target
    maximize_distance: true
    route: safest
  within_feet: 5
  prohibit_willing_departure: true
  additional_save:
    event: damage.applied
    source: enchanted_target
    ability: wisdom
    success: end_effect
```

#### Repeat save

<!-- splat:values path=splat_rule/parameters/repeat_save -->

| Field | Value |
| --- | --- |
| ` event ` | ` turn.end ` |
| ` distance_gt_feet ` | ` 120 ` |
| ` ability ` | ` wisdom ` |
| ` success ` | ` end_effect ` |

#### Successful save immunity

<!-- splat:values path=splat_rule/parameters/successful_save_immunity -->

| Field | Value |
| --- | --- |
| ` minutes ` | ` 1 ` |
| ` scope ` | ` this_effect ` |

#### Mode choice

<!-- splat:values path=splat_rule/parameters/mode_choice -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` caster ` |
| ` options ` | ` ["antipathy", "sympathy"] ` |
| ` at ` | ` cast ` |
