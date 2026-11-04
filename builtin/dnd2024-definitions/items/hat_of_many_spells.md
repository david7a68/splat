---
splat_schema: 2
splat_id: 472db4e4-8f7b-5d50-8859-1b319d5e070b
splat_kind: item
splat_key: dnd2024:items/hat_of_many_spells
splat_name: Hat of Many Spells
splat_origin: dnd2024:reference/magic_item/hat_of_many_spells
splat_sources:
- SRD 5.2.1, pp. 224, 225, https://www.dndbeyond.com/srd
---

# Hat of Many Spells


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Very Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 224 -->

Wondrous Item, Very Rare (Requires Attunement by a Wizard)

This pointed hat has the following properties.

Spellcasting Focus. While holding the hat, you can use it as a Spellcasting Focus for your Wizard spells. Any spell you cast using the hat gains a special Somatic component: you must reach into the hat and “pull” the spell out of it.

<!-- SRD page 225 -->

Unknown Spell. While holding the hat, you can try to cast a level 1+ spell you don’t know. The spell must be on the Wizard spell list, it must be of a level you can cast, and it can’t have Material components costing more than 1,000 GP. Once you decide on the spell, you must expend a spell slot of the spell’s level. Then, to determine whether you cast the spell, make an Intelligence (Arcana) check (DC 10 plus the spell’s level). On a successful check, you cast the spell using its normal casting time, and you can’t use this property again until you finish a Short or Long Rest. On a failed check, you fail to cast the spell and a random effect occurs instead, determined by rolling on the following table.

Any spell you cast from the hat uses your spell save DC and spell attack bonus.

| 1d100 | Effect |
| --- | --- |
| 01–50 | You cast a random spell determined by rolling 1d10: on a 1, Enlarge/Reduce (enlarge effect); on a 2, Enlarge/Reduce (reduce effect); on a 3, Faerie Fire; on a 4, Fireball; on a 5, Gust of Wind; on a 6, Invisibility (cast on yourself); on a 7, Lightning Bolt; on an 8, Phantasmal Force; on a 9, Polymorph; on a 10, Stinking Cloud. |
| 51–55 | You have the Stunned condition until the end of your next turn, believing something awesome just happened. |
| 56–60 | A harmless swarm of butterflies fills a 10-foot Cube within 30 feet of yourself. The swarm disperses after 1 minute. |
| 61–65 | You pull a nonmagical object out of the hat. Roll 1d4 to determine the object: on a 1, a vial of Acid; on a 2, a flask of Alchemist’s Fire; on a 3, a Crowbar; on a 4, a lit Torch. |
| 66–70 | You suffer a bout of “magic sickness” and have the Poisoned condition for 1 hour. |
| 71–75 | You have the Petrified condition until the end of your next turn. |
| 76–80 | You pull a nonmagical object out of the hat. Roll 1d4 to determine the object: on a 1, a Dagger; on a 2, a Rope with a Grappling Hook tied to one end; on a 3, a bag of Caltrops; on a 4, a gem worth 50 GP. |
| 81–85 | A creature appears in an unoccupied space as close to you as possible. The creature isn’t under your control and acts as it normally would, and it disappears after 1 hour or when it drops to 0 Hit Points. Roll 1d4 to determine the creature: on a 1, a Camel; on a 2, a Constrictor Snake; on a 3, an Elephant; on a 4, a Mule. |

| 1d100 | Effect |
| --- | --- |
| 86–90 | A Hostile Swarm of Bats flies out of the hat, occupies your space, and attacks you. |
| 91–95 | A vertical, 10-foot-diameter, two-way portal to another plane of existence opens in an unoccupied space within 30 feet of you and remains open until the end of your next turn. The GM determines where it leads. |
| 96–00 | You pull a magic item out of the hat. Roll 1d6 to determine the item’s rarity: on a 1–3, Common; on a 4–5, Uncommon; on a 6, Rare. The GM chooses the item, which disappears after 1 hour if it’s not consumed or destroyed before then. |

Original layout: [[../../srd-5.2.1/pages/224|page 224]], [[../../srd-5.2.1/pages/225|page 225]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/hat_of_many_spells ` |
| ` implementation ` | ` dnd2024:item_hat_of_many_spells ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` held ` |

#### Focus

<!-- splat:values path=splat_rule/parameters/focus -->

| Field | Value |
| --- | --- |
| ` spell_class ` | ` wizard ` |
| ` added_somatic_component ` | ` reach_into_hat_and_pull_spell_out ` |

#### Unknown spell

<!-- splat:yaml path=splat_rule/parameters/unknown_spell -->

```yaml
selection:
  known: false
  spell_list: wizard
  minimum_level: 1
  level_must_be_castable: true
  maximum_material_component_cost_gp: 1000
slot_cost: selected_spell_level
cost_timing: before_arcana_check
check:
  ability: intelligence
  skill: arcana
  dc:
    op: add
    args:
    - 10
    - binding: selected_spell_level
success:
  cast_selected_spell: true
  casting_time: spell_default
  lockout_until:
  - short_rest_completed
  - long_rest_completed
  slot_cost: already_paid_selected_spell_level
  components: []
failure:
  cast_selected_spell: false
  lockout: false
  table:
    roll:
      expression: 1d100
      scope: use
    rows:
    - min: 1
      max: 50
      result:
        cast_random_spell:
          roll:
            expression: 1d10
            scope: use
          rows:
          - min: 1
            max: 1
            result:
              spell:
                key: dnd2024:spells/enlarge_reduce
              charge_cost: 0
              casting_time: spell_default
              method: item
              slot_cost: 0
              components: []
              level: lowest_possible
              range: spell_default
              duration: spell_default
              concentration: spell_default
              mode: enlarge
          - min: 2
            max: 2
            result:
              spell:
                key: dnd2024:spells/enlarge_reduce
              charge_cost: 0
              casting_time: spell_default
              method: item
              slot_cost: 0
              components: []
              level: lowest_possible
              range: spell_default
              duration: spell_default
              concentration: spell_default
              mode: reduce
          - min: 3
            max: 3
            result:
              spell:
                key: dnd2024:spells/faerie_fire
              charge_cost: 0
              casting_time: spell_default
              method: item
              slot_cost: 0
              components: []
              level: lowest_possible
              range: spell_default
              duration: spell_default
              concentration: spell_default
          - min: 4
            max: 4
            result:
              spell:
                key: dnd2024:spells/fireball
              charge_cost: 0
              casting_time: spell_default
              method: item
              slot_cost: 0
              components: []
              level: lowest_possible
              range: spell_default
              duration: spell_default
              concentration: spell_default
          - min: 5
            max: 5
            result:
              spell:
                key: dnd2024:spells/gust_of_wind
              charge_cost: 0
              casting_time: spell_default
              method: item
              slot_cost: 0
              components: []
              level: lowest_possible
              range: spell_default
              duration: spell_default
              concentration: spell_default
          - min: 6
            max: 6
            result:
              spell:
                key: dnd2024:spells/invisibility
              charge_cost: 0
              casting_time: spell_default
              method: item
              slot_cost: 0
              components: []
              level: lowest_possible
              range: spell_default
              duration: spell_default
              concentration: spell_default
              target: self
          - min: 7
            max: 7
            result:
              spell:
                key: dnd2024:spells/lightning_bolt
              charge_cost: 0
              casting_time: spell_default
              method: item
              slot_cost: 0
              components: []
              level: lowest_possible
              range: spell_default
              duration: spell_default
              concentration: spell_default
          - min: 8
            max: 8
            result:
              spell:
                key: dnd2024:spells/phantasmal_force
              charge_cost: 0
              casting_time: spell_default
              method: item
              slot_cost: 0
              components: []
              level: lowest_possible
              range: spell_default
              duration: spell_default
              concentration: spell_default
          - min: 9
            max: 9
            result:
              spell:
                key: dnd2024:spells/polymorph
              charge_cost: 0
              casting_time: spell_default
              method: item
              slot_cost: 0
              components: []
              level: lowest_possible
              range: spell_default
              duration: spell_default
              concentration: spell_default
          - min: 10
            max: 10
            result:
              spell:
                key: dnd2024:spells/stinking_cloud
              charge_cost: 0
              casting_time: spell_default
              method: item
              slot_cost: 0
              components: []
              level: lowest_possible
              range: spell_default
              duration: spell_default
              concentration: spell_default
    - min: 51
      max: 55
      result:
        condition: stunned
        duration:
          until: end_of_user_next_turn
    - min: 56
      max: 60
      result:
        spawn: harmless_butterfly_swarm
        shape: cube
        size_ft: 10
        range_ft: 30
        duration:
          amount: 1
          unit: minute
    - min: 61
      max: 65
      result:
        nonmagical_item:
          roll:
            expression: 1d4
            scope: use
          rows:
          - min: 1
            max: 1
            result:
              item:
                key: dnd2024:equipment/acid_25_gp
          - min: 2
            max: 2
            result:
              item:
                key: dnd2024:equipment/alchemist_s_fire_50_gp
          - min: 3
            max: 3
            result:
              item:
                key: dnd2024:equipment/crowbar_2_gp
          - min: 4
            max: 4
            result:
              item:
                key: dnd2024:equipment/torch_1_cp
              lit: true
    - min: 66
      max: 70
      result:
        condition: poisoned
        duration:
          amount: 1
          unit: hour
    - min: 71
      max: 75
      result:
        condition: petrified
        duration:
          until: end_of_user_next_turn
    - min: 76
      max: 80
      result:
        nonmagical_item:
          roll:
            expression: 1d4
            scope: use
          rows:
          - min: 1
            max: 1
            result:
              item:
                key: dnd2024:equipment/dagger
          - min: 2
            max: 2
            result:
              items:
              - key: dnd2024:equipment/rope_1_gp
              - key: dnd2024:equipment/grappling_hook_2_gp
              attachment: hook_tied_to_rope_end
          - min: 3
            max: 3
            result:
              item:
                key: dnd2024:equipment/caltrops_1_gp
          - min: 4
            max: 4
            result:
              item_kind: gem
              value_gp: 50
    - min: 81
      max: 85
      result:
        spawn:
          roll:
            expression: 1d4
            scope: use
          rows:
          - min: 1
            max: 1
            result:
              creature:
                key: dnd2024:creatures/camel
          - min: 2
            max: 2
            result:
              creature:
                key: dnd2024:creatures/constrictor_snake
          - min: 3
            max: 3
            result:
              creature:
                key: dnd2024:creatures/elephant
          - min: 4
            max: 4
            result:
              creature:
                key: dnd2024:creatures/mule
        location: closest_unoccupied_space
        controlled: false
        behavior: normal
        duration:
          amount: 1
          unit: hour
        ends:
        - duration_expired
        - zero_hit_points
    - min: 86
      max: 90
      result:
        spawn:
          key: dnd2024:creatures/swarm_of_bats
        location: user_space
        attitude: hostile
        behavior: attack_user
    - min: 91
      max: 95
      result:
        spawn: portal
        orientation: vertical
        diameter_ft: 10
        two_way: true
        location:
          unoccupied: true
          range_ft: 30
        duration:
          until: end_of_user_next_turn
        destination:
          chooser: gm
          type: location
          different_plane: true
    - min: 96
      max: 100
      result:
        spawn_magic_item:
          chooser: gm
          rarity:
            roll:
              expression: 1d6
              scope: use
            rows:
            - min: 1
              max: 3
              result:
                rarity: common
            - min: 4
              max: 5
              result:
                rarity: uncommon
            - min: 6
              max: 6
              result:
                rarity: rare
        disappear_after:
          amount: 1
          unit: hour
        end_early:
        - consumed
        - destroyed
```

#### All hat spells

<!-- splat:values path=splat_rule/parameters/all_hat_spells -->

| Field | Value |
| --- | --- |
| ` save_dc ` | ` user_spell_save_dc ` |
| ` attack_bonus ` | ` user_spell_attack_bonus ` |
| ` added_somatic_component ` | ` reach_into_hat_and_pull_spell_out ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
| ` eligible_classes ` | ` ["wizard"] ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_wondrous_items ` |
| ` 1 ` | ` dnd2024:rules/activating_a_magic_item_spells_cast_from_items ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> equipment:
>   attunement_requirement_text: a Wizard
> ```

