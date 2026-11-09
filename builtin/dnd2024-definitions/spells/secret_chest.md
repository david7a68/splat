---
splat_schema: 2
splat_id: cba41089-70d8-5b5e-98e6-a90af7674f72
splat_kind: spell
splat_key: dnd2024:spells/secret_chest
splat_name: Secret Chest
splat_origin: dnd2024:reference/spell/secret_chest
splat_sources:
- SRD 5.2.1, pp. 160, https://www.dndbeyond.com/srd
---

# Secret Chest


## Spell

| Field | Value |
| --- | --- |
| Level | 4 |
| School | Conjuration |
| Classes | Wizard |
| Casting Time | Action |
| Range |  |
| Components | V, S, M (a chest, 3 feet by 2 feet by 2 feet, constructed from rare materials worth 5,000+ GP, and a Tiny replica of the chest made from the same materials worth 50+ GP) |
| Duration | Until dispelled |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 160 -->

Level 4 Conjuration (Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a chest, 3 feet by 2 feet by 2 feet, constructed from rare materials worth 5,000+ GP, and a Tiny replica of the chest made from the same materials worth 50+ GP)\
Duration: Until dispelled

You hide a chest and all its contents on the Ethereal Plane. You must touch the chest and the miniature replica that serve as Material components for the spell. The chest can contain up to 12 cubic feet of nonliving material (3 feet by 2 feet by 2 feet).

While the chest remains on the Ethereal Plane, you can take a Magic action and touch the replica to recall the chest. It appears in an unoccupied space on the ground within 5 feet of you. You can send the chest back to the Ethereal Plane by taking a Magic action to touch the chest and the replica.

After 60 days, there is a cumulative 5 percent chance at the end of each day that the spell ends. The spell also ends if you cast this spell again or if the Tiny replica chest is destroyed. If the spell ends and the larger chest is on the Ethereal Plane, the chest remains there for you or someone else to find.

Original layout: [[../../srd-5.2.1/pages/160|page 160]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/secret_chest ` |
| ` implementation ` | ` dnd2024:spell_secret_chest ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` additional_end_conditions ` | ` ["caster_casts_secret_chest_again", "replica_destroyed"] ` |

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` until ` | ` dispelled_or_source_specific_end ` |

#### Initial

<!-- splat:yaml path=splat_rule/parameters/initial -->

```yaml
touch_required:
- chest
- replica
destination: ethereal_plane
transported: chest_and_contents
contents:
  nonliving: true
  maximum_cubic_feet: 12
chest_dimensions_feet:
- 3
- 2
- 2
```

#### Recall

<!-- splat:yaml path=splat_rule/parameters/recall -->

```yaml
economy: magic_action
touch_required:
- replica
requires: chest_on_ethereal_plane
destination:
  ground: true
  unoccupied: true
  within_caster_feet: 5
```

#### Send back

<!-- splat:values path=splat_rule/parameters/send_back -->

| Field | Value |
| --- | --- |
| ` economy ` | ` magic_action ` |
| ` touch_required ` | ` ["chest", "replica"] ` |
| ` destination ` | ` ethereal_plane ` |

#### Expiration check

<!-- splat:values path=splat_rule/parameters/expiration_check -->

| Field | Value |
| --- | --- |
| ` starts_after_days ` | ` 60 ` |
| ` event ` | ` day.end ` |
| ` probability_increment_percent_per_day ` | ` 5 ` |
| ` first_eligible_day_percent ` | ` 5 ` |
| ` maximum_percent ` | ` 100 ` |
| ` success ` | ` end_spell ` |

#### End while chest ethereal

<!-- splat:values path=splat_rule/parameters/end_while_chest_ethereal -->

| Field | Value |
| --- | --- |
| ` chest_remains_ethereal ` | ` true ` |
| ` recoverable_by_search ` | ` true ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: touch
> ```

