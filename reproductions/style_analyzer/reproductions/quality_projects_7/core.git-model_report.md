# Model report for file:///tmp/top-repos-quality-repos-x_mp__55/core.git HEAD 674480db75d91bfc265d9ef55ea592452ec394ce

### Dump

```json
{'created_at': '2021-09-01 03:59:17',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '16.9 kB',
 'tags': [],
 'uuid': '55bb2d74-1e17-46b4-90e9-204529e3a605',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-x_mp__55/core.git 674480db75d91bfc265d9ef55ea592452ec394ce

# javascript
61 rules, avg.len. 3.9
## train
PPCR: 0.851098
### report
macro
{'f1-score': 0.6996736788506486,
 'precision': 0.7366687943420092,
 'recall': 0.6808705565023552,
 'support': 5230}
micro
{'f1-score': 0.8487571701720842,
 'precision': 0.8487571701720842,
 'recall': 0.8487571701720842,
 'support': 5230}
weighted
{'f1-score': 0.8417296694947147,
 'precision': 0.8509812646200766,
 'recall': 0.8487571701720842,
 'support': 5230}
### report_full
macro
{'f1-score': 0.6359294873221203,
 'precision': 0.7366687943420092,
 'recall': 0.5832660928705068,
 'support': 6145}
micro
{'f1-score': 0.7804835164835165,
 'precision': 0.8487571701720842,
 'recall': 0.7223759153783564,
 'support': 6145}
weighted
{'f1-score': 0.7614570712172449,
 'precision': 0.8418291415579005,
 'recall': 0.7223759153783564,
 'support': 6145}
## test
PPCR: 0.760390
### report
macro
{'f1-score': 0.4177682409641014,
 'precision': 0.41885997780360823,
 'recall': 0.5897305093302022,
 'support': 2104}
micro
{'f1-score': 0.8141634980988594,
 'precision': 0.8141634980988594,
 'recall': 0.8141634980988594,
 'support': 2104}
weighted
{'f1-score': 0.8079102223890609,
 'precision': 0.8502014815758157,
 'recall': 0.8141634980988594,
 'support': 2104}
### report_full
macro
{'f1-score': 0.36518135708126825,
 'precision': 0.41885997780360823,
 'recall': 0.3733634617163986,
 'support': 2767}
micro
{'f1-score': 0.7033463354547321,
 'precision': 0.8141634980988594,
 'recall': 0.6190820383086375,
 'support': 2767}
weighted
{'f1-score': 0.6692383823786623,
 'precision': 0.8012105577566945,
 'recall': 0.6190820383086375,
 'support': 2767}
```

## javascript
### Summary
34 rules, avg.len. 3.7

| | |
|-|-|
|Min support|142|
|Max support|634|
|Min confidence|0.9203821420669556|
|Max confidence|0.9992113709449768|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
                     'confidence_threshold': 0.8,
                     'max_features': 'auto',
                     'min_samples_leaf': 93,
                     'min_samples_split': 194,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.reserved = ]<br>	∧ -3.diff_line ≥ 1<br>⇒ y = ∅<br>Confidence: 0.997. Support: 160.` |
| 2 | `  -1.reserved not in {]}<br>	∧ -1.roles in {LITERAL}<br>	∧ -3.diff_line ≥ 1<br>⇒ y = "<br>Confidence: 0.999. Support: 472.` |
| 3 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -3.diff_line = 0<br>	∧ -3.label not in {"}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 231.` |
| 4 | `  -2.diff_offset ≥ 3<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 302.` |
| 5 | `  -2.diff_offset ≤ 2<br>	∧ -5.internal_type = StringLiteral<br>	∧ ^2.roles in {LITERAL}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.997. Support: 165.` |
| 6 | `  -1.reserved = :<br>	∧ -1.length ≤ 4<br>⇒ y = ␣<br>Confidence: 0.968. Support: 298.` |
| 7 | `  -1.reserved not in {:}<br>	∧ -1.length ≤ 4<br>	∧ -3.label in {<-space>}<br>	∧ ^2.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.992. Support: 194.` |
| 8 | `  -1.reserved not in {:}<br>	∧ -1.length ≤ 4<br>	∧ -3.diff_offset ≤ 3<br>	∧ -3.label not in {<-space>}<br>	∧ ^2.roles in {MAP}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.948. Support: 182.` |
| 9 | `  -1.reserved = ,<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.986. Support: 183.` |
| 10 | `  -4.diff_line ≥ 1<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {STATEMENT}<br>	∧ ^2.internal_type = ObjectExpression<br>⇒ y = "<br>Confidence: 0.930. Support: 461.` |
| 11 | `  -3.roles in {KEY}<br>	∧ -4.diff_line = 0<br>⇒ y = ␣<br>Confidence: 0.930. Support: 291.` |
| 12 | `  -1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.999. Support: 634.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -5.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IDENTIFIER} and not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 181.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -5.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {CALL, IDENTIFIER}<br>	∧ ^2.roles in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 264.` |
| 15 | `  -2.diff_offset ≥ 3<br>	∧ -3.diff_col ≤ 4<br>	∧ +2.roles not in {STRING}<br>	∧ ^1.roles in {INITIALIZATION}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = "<br>Confidence: 0.945. Support: 282.` |
| 16 | `  -1.roles not in {LITERAL}<br>	∧ -1.length ≥ 3<br>	∧ -2.diff_offset ≥ 3<br>	∧ +1.length ≥ 3<br>	∧ ^1.roles not in {CALL, INITIALIZATION}<br>	∧ ^2.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 160.` |
| 17 | `  -2.diff_offset ≤ 2<br>	∧ -4.diff_offset ≥ 9<br>	∧ +5.roles in {MAP}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 240.` |
| 18 | `  -2.diff_offset ≤ 2<br>	∧ -4.diff_offset ≤ 8<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.968. Support: 265.` |
| 19 | `  -1.roles in {LITERAL}<br>	∧ -4.diff_offset ≥ 15<br>⇒ y = "<br>Confidence: 0.984. Support: 589.` |
| 20 | `  -1.roles not in {LITERAL}<br>	∧ -5.diff_offset ≥ 16<br>	∧ +1.roles in {MAP}<br>	∧ ^2.internal_type = ObjectProperty<br>⇒ y = "<br>Confidence: 0.940. Support: 193.` |
| 21 | `  -1.roles not in {LITERAL}<br>	∧ -2.internal_type = StringLiteral<br>	∧ +1.roles not in {MAP}<br>	∧ ^2.internal_type = ObjectProperty<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.996. Support: 142.` |
| 22 | `  -1.roles not in {LITERAL}<br>	∧ -3.label in {<space>}<br>	∧ -5.roles in {KEY}<br>	∧ ^2.internal_type not in {ObjectProperty}<br>⇒ y = "<br>Confidence: 0.942. Support: 180.` |
| 23 | `  -1.roles not in {LITERAL}<br>	∧ -3.label not in {<space>}<br>	∧ -5.roles in {KEY}<br>	∧ ^2.internal_type not in {ObjectProperty}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.983. Support: 200.` |
| 24 | `  -1.roles not in {LITERAL}<br>	∧ -5.roles not in {KEY}<br>	∧ +1.reserved = [<br>	∧ ^2.internal_type not in {ObjectProperty}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 184.` |
| 25 | `  -1.roles not in {LITERAL}<br>	∧ -5.roles not in {KEY}<br>	∧ +1.reserved not in {[}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {IDENTIFIER}<br>	∧ ^2.internal_type not in {ObjectProperty}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 165.` |
| 26 | `  -3.diff_line ≥ 1<br>	∧ ^2.internal_type = ObjectExpression<br>⇒ y = "<br>Confidence: 0.976. Support: 445.` |
| 27 | `  -2.diff_line ≥ 1<br>	∧ -3.diff_line ≥ 1<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = "<br>Confidence: 0.998. Support: 296.` |
| 28 | `  -1.reserved = [<br>	∧ -3.diff_line = 0<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.944. Support: 186.` |
| 29 | `  -1.reserved = :<br>	∧ -3.diff_line = 0<br>⇒ y = ␣<br>Confidence: 0.967. Support: 287.` |
| 30 | `  -1.label in {"}<br>	∧ -1.reserved not in {,, :, [}<br>	∧ -3.diff_line = 0<br>	∧ ^1.internal_type not in {Program}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.920. Support: 157.` |
| 31 | `  -4.diff_offset ≤ 5<br>	∧ -4.label in {"}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.986. Support: 184.` |
| 32 | `  -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.length ≤ 10<br>	∧ ^1.internal_type = ObjectProperty<br>⇒ y = ␣<br>Confidence: 0.959. Support: 233.` |
| 33 | `  -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.length ≤ 10<br>	∧ ^1.internal_type not in {ObjectProperty}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 219.` |
| 34 | `  -1.diff_col ≥ 2<br>	∧ ^2.internal_type = ObjectExpression<br>⇒ y = "<br>Confidence: 0.958. Support: 444.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.7058823529411766, "max_conf": 0.9992113709449768, "max_support": 634, "min_conf": 0.9203821420669556, "min_support": 142, "num_rules": 34}}
```
</details>
