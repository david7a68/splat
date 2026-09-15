---
splat_schema: 1
splat_id: 032e1ba9-a324-5547-aae0-6c6793e87061
splat_kind: item
splat_key: dnd2024:items/animated_shield
splat_name: Animated Shield
splat_origin:
  key: dnd2024:reference/magic_item/animated_shield
splat_sources:
- title: SRD 5.2.1
  locator: pp. 209
  url: https://www.dndbeyond.com/srd
---

# Animated Shield

## Source text

<!-- SRD page 209 -->

Armor (Shield), Very Rare (Requires Attunement)\
While holding this Shield, you can take a Bonus\
Action to cause it to animate. The Shield leaps into\
the air and hovers in your space to protect you as if\
you were wielding it, leaving your hands free. The\
Shield remains animate for 1 minute, until you take\
a Bonus Action to end this effect, or until you die or\
have the Incapacitated condition, at which point the\
Shield falls to the ground or into your hand if you\
have one free.\

Original layout: [[../../srd-5.2.1/pages/209|page 209]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` armor ` |
| ` rarity ` | ` very_rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/animated_shield ` |
| ` implementation ` | ` dnd2024:item_animated_shield ` |

### Parameters

#### Activation

<!-- splat:yaml path=splat_rule/parameters/activation -->

```yaml
action: bonus_action
requires: holding
duration:
  value: 1
  unit: minute
shield_state: animated
location: wielder_space
protection: as_wielded
hands_required: 0
end_on:
- duration
- bonus_action_end
- wielder_death
- wielder_incapacitated
on_end:
  if_free_hand: return_to_hand
  otherwise: fall_to_ground
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
