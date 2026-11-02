---
splat_schema: 1
splat_id: 104ffb52-c4d5-50cf-8ef5-4bc801b8a714
splat_kind: spell
splat_key: dnd2024:spells/unseen_servant
splat_name: Unseen Servant
splat_origin:
  key: dnd2024:reference/spell/unseen_servant
splat_sources:
- title: SRD 5.2.1
  locator: pp. 171
  url: https://www.dndbeyond.com/srd
---

# Unseen Servant

## Source text

<!-- SRD page 171 -->

Level 1 Conjuration (Bard, Warlock, Wizard)\
Casting Time: Action or Ritual\
Range: 60 feet\
Components: V, S, M (a bit of string and of wood)\
Duration: 1 hour

This spell creates an Invisible, mindless, shapeless, Medium force that performs simple tasks at your command until the spell ends. The servant springs into existence in an unoccupied space on the ground within range. It has AC 10, 1 Hit Point, and a Strength of 2, and it can’t attack. If it drops to 0 Hit Points, the spell ends.

Once on each of your turns as a Bonus Action, you can mentally command the servant to move up to 15 feet and interact with an object. The servant can perform simple tasks that a human could do, such as fetching things, cleaning, mending, folding clothes, lighting fires, serving food, and pouring drinks. Once you give the command, the servant performs the task to the best of its ability until it completes the task, then waits for your next command.

If you command the servant to perform a task that would move it more than 60 feet away from you, the spell ends.

Original layout: [[../../srd-5.2.1/pages/171|page 171]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` conjuration ` |
| ` classes ` | ` ["bard", "warlock", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 1 hour ` |
| ` casting_time ` | ` Action or Ritual ` |
| ` ritual ` | ` true ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 60 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a bit of string and of wood ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/unseen_servant ` |
| ` implementation ` | ` dnd2024:spell_unseen_servant ` |

### Parameters

#### Force

<!-- splat:values path=splat_rule/parameters/force -->

| Field | Value |
| --- | --- |
| ` size ` | ` medium ` |
| ` invisible ` | ` true ` |
| ` mindless ` | ` true ` |
| ` shapeless ` | ` true ` |
| ` initial_space ` | ` unoccupied_ground_in_range ` |
| ` ac ` | ` 10 ` |
| ` hp ` | ` 1 ` |
| ` strength ` | ` 2 ` |
| ` can_attack ` | ` false ` |

#### End

<!-- splat:values path=splat_rule/parameters/end -->

| Field | Value |
| --- | --- |
| ` hp_zero ` | ` true ` |
| ` command_requires_move_beyond_caster_feet ` | ` 60 ` |

#### Command

<!-- splat:values path=splat_rule/parameters/command -->

| Field | Value |
| --- | --- |
| ` cost ` | ` bonus_action ` |
| ` maximum_per_caster_turn ` | ` 1 ` |
| ` mental ` | ` true ` |
| ` move_maximum_feet ` | ` 15 ` |
| ` object_interaction ` | ` true ` |
| ` task_scope ` | ` simple_human_tasks ` |
| ` examples ` | ` ["fetch", "clean", "mend", "fold_clothes", "light_fire", "serve_food", "pour_drink"] ` |
| ` continue ` | ` best_ability_until_complete ` |
| ` after_complete ` | ` wait_for_command ` |
