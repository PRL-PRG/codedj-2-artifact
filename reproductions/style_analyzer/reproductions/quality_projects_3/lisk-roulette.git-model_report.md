# Model report for file:///tmp/top-repos-quality-repos-_ojbtj4o/lisk-roulette.git HEAD 4f30d12e9c770f1a1d532b91e56271091ed3cd83

### Dump

```json
{'created_at': '2021-09-02 07:14:33',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-74-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '17.2 kB',
 'tags': [],
 'uuid': '8fec9b43-8798-47b1-9948-39fac42bdade',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-_ojbtj4o/lisk-roulette.git 4f30d12e9c770f1a1d532b91e56271091ed3cd83

# javascript
43 rules, avg.len. 5.4
## train
PPCR: 0.900095
### report
macro
{'f1-score': 0.4882291194987436,
 'precision': 0.5494426674578686,
 'recall': 0.4572432581935919,
 'support': 16100}
micro
{'f1-score': 0.9151552795031055,
 'precision': 0.9151552795031056,
 'recall': 0.9151552795031056,
 'support': 16100}
weighted
{'f1-score': 0.9020999760003021,
 'precision': 0.8933110739484369,
 'recall': 0.9151552795031056,
 'support': 16100}
### report_full
macro
{'f1-score': 0.42382104065180337,
 'precision': 0.5494426674578686,
 'recall': 0.3823468293266179,
 'support': 17887}
micro
{'f1-score': 0.8670373966516609,
 'precision': 0.9151552795031056,
 'recall': 0.8237267289092637,
 'support': 17887}
weighted
{'f1-score': 0.8289609359282771,
 'precision': 0.860475346999305,
 'recall': 0.8237267289092637,
 'support': 17887}
## test
PPCR: 0.852782
### report
macro
{'f1-score': 0.32926011808053757,
 'precision': 0.3358837456750806,
 'recall': 0.3476242240077373,
 'support': 3157}
micro
{'f1-score': 0.8102629078238834,
 'precision': 0.8102629078238834,
 'recall': 0.8102629078238834,
 'support': 3157}
weighted
{'f1-score': 0.7669202737114236,
 'precision': 0.73641920962263,
 'recall': 0.8102629078238834,
 'support': 3157}
### report_full
macro
{'f1-score': 0.2999283950299097,
 'precision': 0.3358837456750806,
 'recall': 0.28673247049604655,
 'support': 3702}
micro
{'f1-score': 0.7458813238081353,
 'precision': 0.8102629078238834,
 'recall': 0.690977849810913,
 'support': 3702}
weighted
{'f1-score': 0.6770723753197079,
 'precision': 0.668083251151156,
 'recall': 0.690977849810913,
 'support': 3702}
```

## javascript
### Summary
13 rules, avg.len. 5.4

| | |
|-|-|
|Min support|144|
|Max support|9457|
|Min confidence|0.944961428642273|
|Max confidence|0.9976851940155029|

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
                     'max_depth': 10,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.reserved = ,<br>	∧ +1.roles in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 478.` |
| 2 | `  -1.reserved not in {,}<br>	∧ -4.diff_line ≥ 2<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 166.` |
| 3 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.997. Support: 168.` |
| 4 | `  •••start_col ≥ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 159.` |
| 5 | `  •••start_col ≥ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 9457.` |
| 6 | `  -1.reserved not in {,}<br>	∧ -4.diff_line ≥ 2<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 157.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 216.` |
| 8 | `  -1.reserved not in {,}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -4.diff_line ≥ 2<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 144.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 167.` |
| 10 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.997. Support: 179.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -4.diff_line ≥ 2<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 144.` |
| 12 | `  •••start_col ≥ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 8892.` |
| 13 | `  -1.reserved not in {,}<br>	∧ -3.diff_col ≥ 4<br>	∧ -4.diff_line ≥ 2<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 160.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.384615384615385, "max_conf": 0.9976851940155029, "max_support": 9457, "min_conf": 0.944961428642273, "min_support": 144, "num_rules": 13}}
```
</details>
