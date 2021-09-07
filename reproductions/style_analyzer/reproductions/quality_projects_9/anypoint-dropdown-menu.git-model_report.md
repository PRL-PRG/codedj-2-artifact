# Model report for file:///tmp/top-repos-quality-repos-f0r11jdy/anypoint-dropdown-menu.git HEAD e02e5909d6b3a21d0b93174448da9b7a4cbc63ce

### Dump

```json
{'created_at': '2021-08-31 20:14:55',
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
 'size': '15.7 kB',
 'tags': [],
 'uuid': '7da3721d-e3c9-4575-8136-bb4f1d82620a',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-f0r11jdy/anypoint-dropdown-menu.git e02e5909d6b3a21d0b93174448da9b7a4cbc63ce

# javascript
13 rules, avg.len. 5.1
## train
PPCR: 0.855675
### report
macro
{'f1-score': 0.9317014196670088,
 'precision': 0.9508328660443012,
 'recall': 0.9143563754100087,
 'support': 7909}
micro
{'f1-score': 0.9323555443166014,
 'precision': 0.9323555443166014,
 'recall': 0.9323555443166014,
 'support': 7909}
weighted
{'f1-score': 0.9317494600650259,
 'precision': 0.9322370582350418,
 'recall': 0.9323555443166014,
 'support': 7909}
### report_full
macro
{'f1-score': 0.8143286805439648,
 'precision': 0.9508328660443012,
 'recall': 0.7225574300614611,
 'support': 9243}
micro
{'f1-score': 0.8598414179104478,
 'precision': 0.9323555443166014,
 'recall': 0.7977929243752029,
 'support': 9243}
weighted
{'f1-score': 0.8555983858583031,
 'precision': 0.9328143377083282,
 'recall': 0.7977929243752029,
 'support': 9243}
## test
PPCR: 0.866667
### report
macro
{'f1-score': 0.40163265306122453,
 'precision': 0.40389610389610386,
 'recall': 0.40544217687074824,
 'support': 39}
micro
{'f1-score': 0.6153846153846154,
 'precision': 0.6153846153846154,
 'recall': 0.6153846153846154,
 'support': 39}
weighted
{'f1-score': 0.6326739926739927,
 'precision': 0.6585081585081585,
 'recall': 0.6153846153846154,
 'support': 39}
### report_full
macro
{'f1-score': 0.3608734123019838,
 'precision': 0.40389610389610386,
 'recall': 0.33758503401360546,
 'support': 45}
micro
{'f1-score': 0.5714285714285715,
 'precision': 0.6153846153846154,
 'recall': 0.5333333333333333,
 'support': 45}
weighted
{'f1-score': 0.5921403041403042,
 'precision': 0.6773737373737373,
 'recall': 0.5333333333333333,
 'support': 45}
```

## javascript
### Summary
10 rules, avg.len. 4.5

| | |
|-|-|
|Min support|106|
|Max support|2436|
|Min confidence|0.9345238208770752|
|Max confidence|0.9984025359153748|

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
                     'min_samples_leaf': 90,
                     'min_samples_split': 190,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.998. Support: 206.` |
| 2 | `  •••start_col ≥ 13<br>	∧ -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>⇒ y = ⏎<br>Confidence: 0.955. Support: 300.` |
| 3 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = '<br>Confidence: 0.995. Support: 106.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 2436.` |
| 5 | `  -1.reserved = {<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.968. Support: 231.` |
| 6 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;, {}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = '<br>Confidence: 0.997. Support: 183.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = '<br>Confidence: 0.974. Support: 134.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 313.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 374.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved = )<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 110.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.5, "max_conf": 0.9984025359153748, "max_support": 2436, "min_conf": 0.9345238208770752, "min_support": 106, "num_rules": 10}}
```
</details>
