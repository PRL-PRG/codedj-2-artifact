# Model report for file:///tmp/top-repos-quality-repos-sq94abof/jsonservercgivisitkort.git HEAD 4e24abd73073984c8d84968f5aa51f6d443f1be5

### Dump

```json
{'created_at': '2021-09-02 05:04:05',
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
 'size': '16.4 kB',
 'tags': [],
 'uuid': '971d1458-dc7f-47b9-b92b-348b38785d1f',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-sq94abof/jsonservercgivisitkort.git 4e24abd73073984c8d84968f5aa51f6d443f1be5

# javascript
18 rules, avg.len. 7.1
## train
PPCR: 0.893574
### report
macro
{'f1-score': 0.8899709352021927,
 'precision': 0.9253202053015787,
 'recall': 0.8668085364233432,
 'support': 12863}
micro
{'f1-score': 0.9511000544196533,
 'precision': 0.9511000544196533,
 'recall': 0.9511000544196533,
 'support': 12863}
weighted
{'f1-score': 0.9503617384972246,
 'precision': 0.9525490829917935,
 'recall': 0.9511000544196533,
 'support': 12863}
### report_full
macro
{'f1-score': 0.7693975922122503,
 'precision': 0.9253202053015787,
 'recall': 0.6921345038385115,
 'support': 14395}
micro
{'f1-score': 0.8976447281532027,
 'precision': 0.9511000544196533,
 'recall': 0.8498784300104203,
 'support': 14395}
weighted
{'f1-score': 0.8858978610810707,
 'precision': 0.948321077326804,
 'recall': 0.8498784300104203,
 'support': 14395}
## test
PPCR: 0.922348
### report
macro
{'f1-score': 0.8505240120562977,
 'precision': 0.8751009408596545,
 'recall': 0.8372634157010798,
 'support': 2043}
micro
{'f1-score': 0.9388154674498287,
 'precision': 0.9388154674498287,
 'recall': 0.9388154674498287,
 'support': 2043}
weighted
{'f1-score': 0.9363806927608531,
 'precision': 0.9367781130682219,
 'recall': 0.9388154674498287,
 'support': 2043}
### report_full
macro
{'f1-score': 0.7984004944358783,
 'precision': 0.8751009408596545,
 'recall': 0.7637441964808637,
 'support': 2215}
micro
{'f1-score': 0.9008924377642085,
 'precision': 0.9388154674498287,
 'recall': 0.8659142212189617,
 'support': 2215}
weighted
{'f1-score': 0.8882715527378975,
 'precision': 0.9254260130429869,
 'recall': 0.8659142212189617,
 'support': 2215}
```

## javascript
### Summary
13 rules, avg.len. 6.8

| | |
|-|-|
|Min support|91|
|Max support|3003|
|Min confidence|0.9599406719207764|
|Max confidence|0.9995173811912537|

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
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 91,
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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.999. Support: 579.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.995. Support: 110.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.998. Support: 300.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.983. Support: 3003.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = .<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1036.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 894.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, .}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>⇒ y = ∅<br>Confidence: 0.960. Support: 337.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {MAP}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.982. Support: 244.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>⇒ y = ∅<br>Confidence: 0.997. Support: 182.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, [, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>⇒ y = ∅<br>Confidence: 0.996. Support: 116.` |
| 11 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ., [, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ., >, }}<br>	∧ ^1.roles in {SCOPE}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 125.` |
| 12 | `  •••start_col ≤ 14<br>	∧ -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ., [, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ., >, }}<br>	∧ ^1.roles in {SCOPE}<br>⇒ y = ⏎⏎<br>Confidence: 0.972. Support: 124.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, [, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, ., }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {SCOPE}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 91.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.769230769230769, "max_conf": 0.9995173811912537, "max_support": 3003, "min_conf": 0.9599406719207764, "min_support": 91, "num_rules": 13}}
```
</details>
