---
splat_schema: 2
splat_id: 171111ce-3d96-5c9f-99f9-7d39e7f2bb37
splat_kind: spell
splat_key: dnd2024:spells/knock
splat_name: Knock
splat_origin: dnd2024:reference/spell/knock
splat_sources:
- SRD 5.2.1, pp. 143, 144, https://www.dndbeyond.com/srd
---

# Knock


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Transmutation |
| Classes | Bard, Sorcerer, Wizard |
| Casting Time | Action |
| Range | 60 feet |
| Components | V |
| Duration | Instantaneous |


## Source text

<!-- SRD page 143 -->

Level 2 Transmutation (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V\
Duration: Instantaneous

<!-- SRD page 144 -->

Choose an object that you can see within range. The object can be a door, a box, a chest, a set of manacles, a padlock, or another object that contains a mundane or magical means that prevents access.

A target that is held shut by a mundane lock or that is stuck or barred becomes unlocked, unstuck, or unbarred. If the object has multiple locks, only one of them is unlocked.

If the target is held shut by Arcane Lock, that spell is suppressed for 10 minutes, during which time the target can be opened and closed.

When you cast the spell, a loud knock, audible up to 300 feet away, emanates from the target.

Original layout: [[../../srd-5.2.1/pages/143|page 143]], [[../../srd-5.2.1/pages/144|page 144]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/knock ` |
| ` implementation ` | ` dnd2024:spell_knock ` |

### Parameters

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` access_preventing_object ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` true ` |

#### Mundane

<!-- splat:values path=splat_rule/parameters/mundane -->

| Field | Value |
| --- | --- |
| ` unlock_locks_maximum ` | ` 1 ` |
| ` unstick ` | ` true ` |
| ` unbar ` | ` true ` |

#### Arcane lock

<!-- splat:values path=splat_rule/parameters/arcane_lock -->

| Field | Value |
| --- | --- |
| ` suppress_minutes ` | ` 10 ` |
| ` can_open_and_close ` | ` true ` |

#### Sound

<!-- splat:values path=splat_rule/parameters/sound -->

| Field | Value |
| --- | --- |
| ` origin ` | ` target ` |
| ` audible_feet ` | ` 300 ` |
