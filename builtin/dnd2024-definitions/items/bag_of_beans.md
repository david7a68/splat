---
splat_schema: 1
splat_id: 9591a122-382b-5603-a720-a40c1093899e
splat_kind: item
splat_key: dnd2024:items/bag_of_beans
splat_name: Bag of Beans
splat_origin:
  key: dnd2024:reference/magic_item/bag_of_beans
splat_sources:
- title: SRD 5.2.1
  locator: pp. 211
  url: https://www.dndbeyond.com/srd
---

# Bag of Beans

## Source text

<!-- SRD page 211 -->

Wondrous Item, Rare

This heavy cloth bag contains 3d4 dry beans when found. The bag weighs half a pound regardless of how many beans it contains and becomes a nonmagical item when it no longer contains any beans.

If you dump one or more beans out of the bag, they explode in a 10-foot-radius Sphere centered on them. All the dumped beans are destroyed in the explosion, and each creature in the Sphere, including you, makes a DC 15 Dexterity saving throw, taking 5d4 Force damage on a failed save or half as much damage on a successful one.

If you remove a bean from the bag, plant it in dirt or sand, and then water it, the bean disappears as it produces an effect 1 minute later from the ground where it was planted. The GM can choose an effect from the following table or determine it randomly.

| 1d100 | Effect |
| --- | --- |
| 01 | 5d4 toadstools sprout. If a creature eats a toadstool, roll any die. On an odd roll, the eater must succeed on a DC 15 Constitution saving throw or take 5d6 Poison damage and have the Poisoned condition for 1 hour. On an even roll, the eater gains 5d6 Temporary Hit Points for 1 hour. |
| 02–10 | A geyser erupts and spouts water, beer, mayonnaise, tea, vinegar, wine, or oil (GM’s choice) 30 feet into the air for 1d4 minutes. |

| 1d100 | Effect |
| --- | --- |
| 11–20 | A Treant sprouts. Roll any die. On an odd roll, the treant is Chaotic Evil. On an even roll, the treant is Chaotic Good. |
| 21–30 | An animate but immobile stone statue in your likeness rises and makes verbal threats against you. If you leave it and others come near, it describes you as the most heinous of villains and directs the newcomers to find and attack you. If you are on the same plane of existence as the statue, it knows where you are. The statue becomes inanimate after 24 hours. |
| 31–40 | A campfire with green flames springs forth and burns for 24 hours or until it is extinguished. |
| 41–50 | Three Shrieker Fungi sprout. |
| 51–60 | 1d4 + 4 bright-pink toads crawl forth. Whenever a toad is touched, it transforms into a Large or smaller monster of the GM’s choice that acts in accordance with its alignment and nature. The monster remains for 1 minute, then disappears in a puff of bright-pink smoke. |
| 61–70 | A hungry Bulette burrows up and attacks. |
| 71–80 | A fruit tree grows. It has 1d10 + 20 fruit, 1d8 of which act as randomly determined potions. The tree vanishes after 1 hour. Picked fruit remains, retaining any magic for 30 days. |
| 81–90 | A nest of 1d4 + 3 rainbow-colored eggs springs up. Any creature that eats an egg makes a DC 20 Constitution saving throw. On a successful save, a creature permanently increases its lowest ability score by 1, randomly choosing among equally low scores. On a failed save, the creature takes 10d6 Force damage from an internal explosion. |
| 91–95 | A pyramid with a 60-foot-square base bursts upward. Inside is a burial chamber containing a Mummy, a Mummy Lord, or some other Undead of the GM’s choice. Its sarcophagus contains treasure of the GM’s choice. |
| 96–00 | A giant beanstalk sprouts, growing to a height of the GM’s choice. The top leads where the GM chooses, such as to a great view, a cloud giant’s castle, or another plane of existence. |

Original layout: [[../../srd-5.2.1/pages/211|page 211]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/bag_of_beans ` |
| ` implementation ` | ` dnd2024:item_bag_of_beans ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` weight_lb ` | ` 0.5 ` |
| ` weight_independent_of_contents ` | ` true ` |

#### Beans

<!-- splat:yaml path=splat_rule/parameters/beans -->

```yaml
initial:
  expression: 3d4
  scope: use
on_empty:
  magical: false
```

#### Dump

<!-- splat:yaml path=splat_rule/parameters/dump -->

```yaml
minimum_beans: 1
destroy: all_dumped_beans
area:
  shape: sphere
  radius_ft: 10
  origin: beans
  include_actor: true
save:
  ability: dexterity
  dc: 15
  failure:
    damage:
      roll:
        expression: 5d4
        scope: use
      type: force
  success:
    damage_scale:
      numerator: 1
      denominator: 2
```

#### Plant

<!-- splat:yaml path=splat_rule/parameters/plant -->

```yaml
cost_beans: 1
substrates:
- dirt
- sand
water_required: true
consume_bean: true
delay:
  amount: 1
  unit: minute
origin: planted_location
chooser: gm
method:
- choose
- roll
table:
  roll:
    expression: 1d100
    scope: use
  rows:
  - min: 1
    max: 1
    result:
      spawn: toadstools
      count:
        expression: 5d4
        scope: use
      on_eat:
        roll_parity:
          odd:
            save:
              ability: constitution
              dc: 15
              failure:
                damage:
                  roll:
                    expression: 5d6
                    scope: use
                  type: poison
                condition: poisoned
                duration:
                  amount: 1
                  unit: hour
              success:
                effect: none
          even:
            temporary_hp:
              expression: 5d6
              scope: use
            duration:
              amount: 1
              unit: hour
  - min: 2
    max: 10
    result:
      spawn: geyser
      height_ft: 30
      duration:
        roll:
          expression: 1d4
          scope: use
        unit: minute
      fluid:
        chooser: gm
        options:
        - water
        - beer
        - mayonnaise
        - tea
        - vinegar
        - wine
        - oil
  - min: 11
    max: 20
    result:
      spawn:
        key: dnd2024:creatures/treant
      alignment_by_roll_parity:
        odd: chaotic_evil
        even: chaotic_good
  - min: 21
    max: 30
    result:
      spawn: stone_statue
      appearance: planter
      animate: true
      mobile: false
      behavior:
      - threaten_planter
      - if_planter_absent_describe_planter_as_villain
      - direct_visitors_to_find_and_attack_planter
      track_planter:
        same_plane_required: true
        exact_location_known: true
      becomes_inanimate_after:
        amount: 24
        unit: hour
  - min: 31
    max: 40
    result:
      spawn: green_campfire
      duration:
        amount: 24
        unit: hour
      ends_on: extinguished
  - min: 41
    max: 50
    result:
      spawn:
        key: dnd2024:creatures/shrieker_fungus
      count: 3
  - min: 51
    max: 60
    result:
      spawn: pink_toads
      count:
        expression: 1d4+4
        scope: use
      on_touch:
        transform:
          chooser: gm
          definition_category: creature
          maximum_size: large
        behavior: alignment_and_nature
        disappear_after:
          amount: 1
          unit: minute
  - min: 61
    max: 70
    result:
      spawn:
        key: dnd2024:creatures/bulette
      entry: burrow_up
      behavior: attack
  - min: 71
    max: 80
    result:
      spawn: fruit_tree
      fruit_count:
        expression: 1d10+20
        scope: use
      magic_fruit_count:
        expression: 1d8
        scope: use
      magic_fruit:
        selection: random
        definition_category: potion
        effect: selected_potion
      tree_vanishes_after:
        amount: 1
        unit: hour
      picked_fruit_remains: true
      picked_magic_expires_after:
        amount: 30
        unit: day
  - min: 81
    max: 90
    result:
      spawn: rainbow_eggs
      count:
        expression: 1d4+3
        scope: use
      on_eat:
        save:
          ability: constitution
          dc: 20
          failure:
            damage:
              roll:
                expression: 10d6
                scope: use
              type: force
          success:
            ability_score_increase: 1
            ability_selector: lowest
            tie_breaker: random
            duration: permanent
  - min: 91
    max: 95
    result:
      spawn: pyramid
      base_square_ft: 60
      interior: burial_chamber
      occupant:
        chooser: gm
        options:
        - key: dnd2024:creatures/mummy
        - key: dnd2024:creatures/mummy_lord
        additional_allowed_creature_type: undead
      sarcophagus_treasure:
        chooser: gm
        type: item_list
  - min: 96
    max: 100
    result:
      spawn: giant_beanstalk
      height_ft:
        chooser: gm
        type: number
      top_destination:
        chooser: gm
        type: location
        allows_other_planes: true
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_wondrous_items ` |
