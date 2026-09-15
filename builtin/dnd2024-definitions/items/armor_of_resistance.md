---
splat_schema: 1
splat_id: 74b311c4-16a8-53a2-a8bf-e375602b61e8
splat_kind: item
splat_key: dnd2024:items/armor_of_resistance
splat_name: Armor of Resistance
splat_origin:
  key: dnd2024:reference/magic_item/armor_of_resistance
splat_sources:
- title: SRD 5.2.1
  locator: pp. 210
  url: https://www.dndbeyond.com/srd
---

# Armor of Resistance

## Source text

<!-- SRD page 210 -->

Armor (Any Light, Medium, or Heavy), Rare (Requires\
Attunement)\
You have Resistance to one type of damage while\
you wear this armor. The GM chooses the type or\
determines it randomly by rolling on the following\
table.\
1d10 Damage Type 1d10 Damage Type\
1 Acid 6 Necrotic\
2 Cold 7 Poison\
3 Fire 8 Psychic\
4 Force 9 Radiant\
5 Lightning 10 Thunder\
Down\
Speed to 0 and making it unable to benefit from bo-\
nuses to speed.\
Forward window shutter closes.\
Side window shutters close (two per side).\
The claws retract.\
Each extended claw makes the following melee attack:\
+8 to hit, reach 5 ft. Hit: The target has the Grappled\
condition (escape DC 15).\
The apparatus walks or swims backward provided its\
legs are extended.\
The apparatus turns 90 degrees clockwise provided its\
legs are extended.\
The light turns off.\
The apparatus rises up to 20 feet if it’s in liquid.\
The rear hatch closes and seals.\

Original layout: [[../../srd-5.2.1/pages/210|page 210]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` armor ` |
| ` rarity ` | ` rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/armor_of_resistance ` |
| ` implementation ` | ` dnd2024:item_armor_of_resistance ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |

#### Resistance

<!-- splat:yaml path=splat_rule/parameters/resistance -->

```yaml
chooser: gm
method:
- choose
- roll
table:
  roll:
    expression: 1d10
    scope: use
  rows:
  - min: 1
    max: 1
    result:
      damage_type: acid
  - min: 2
    max: 2
    result:
      damage_type: cold
  - min: 3
    max: 3
    result:
      damage_type: fire
  - min: 4
    max: 4
    result:
      damage_type: force
  - min: 5
    max: 5
    result:
      damage_type: lightning
  - min: 6
    max: 6
    result:
      damage_type: necrotic
  - min: 7
    max: 7
    result:
      damage_type: poison
  - min: 8
    max: 8
    result:
      damage_type: psychic
  - min: 9
    max: 9
    result:
      damage_type: radiant
  - min: 10
    max: 10
    result:
      damage_type: thunder
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_armor ` |
