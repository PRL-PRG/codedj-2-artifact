# Model report for file:///tmp/top-repos-quality-repos-yah1jnco/slate.git HEAD 4f329c6890e72bea03641665569db9a93a8e0f1e

### Dump

```json
{'created_at': '2021-09-02 00:34:29',
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
 'size': '16.7 kB',
 'tags': [],
 'uuid': '43de820a-cbff-46f1-a90c-15f08e1c26d8',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-yah1jnco/slate.git 4f329c6890e72bea03641665569db9a93a8e0f1e

# javascript
12 rules, avg.len. 6.5
## train
PPCR: 0.783301
### report
macro
{'f1-score': 0.6669562657472305,
 'precision': 0.6855217896251682,
 'recall': 0.6520433489650754,
 'support': 7768}
micro
{'f1-score': 0.9214727085478887,
 'precision': 0.9214727085478888,
 'recall': 0.9214727085478888,
 'support': 7768}
weighted
{'f1-score': 0.9142016546139148,
 'precision': 0.9091517426346973,
 'recall': 0.9214727085478888,
 'support': 7768}
### report_full
macro
{'f1-score': 0.5709244965430481,
 'precision': 0.6855217896251682,
 'recall': 0.5163381395176323,
 'support': 9917}
micro
{'f1-score': 0.8094995759117897,
 'precision': 0.9214727085478888,
 'recall': 0.7217908641726328,
 'support': 9917}
weighted
{'f1-score': 0.7828479983600314,
 'precision': 0.8798030834358024,
 'recall': 0.7217908641726328,
 'support': 9917}
## test
PPCR: 0.764774
### report
macro
{'f1-score': 0.5110289158224564,
 'precision': 0.4980603448275862,
 'recall': 0.527117500700869,
 'support': 660}
micro
{'f1-score': 0.9075757575757576,
 'precision': 0.9075757575757576,
 'recall': 0.9075757575757576,
 'support': 660}
weighted
{'f1-score': 0.8918923635095231,
 'precision': 0.8787409812409812,
 'recall': 0.9075757575757576,
 'support': 660}
### report_full
macro
{'f1-score': 0.4703659705074273,
 'precision': 0.4980603448275862,
 'recall': 0.44984464819846326,
 'support': 863}
micro
{'f1-score': 0.7866053841103086,
 'precision': 0.9075757575757576,
 'recall': 0.694090382387022,
 'support': 863}
weighted
{'f1-score': 0.737918446320756,
 'precision': 0.7940569898795016,
 'recall': 0.694090382387022,
 'support': 863}
```

## javascript
### Summary
5 rules, avg.len. 5.4

| | |
|-|-|
|Min support|124|
|Max support|1753|
|Min confidence|0.9278728365898132|
|Max confidence|0.9967391490936279|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.985. Support: 1753.` |
| 2 | `  ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.928. Support: 409.` |
| 3 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 460.` |
| 4 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = {<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.950. Support: 190.` |
| 5 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.diff_col ≤ 3<br>	∧ -2.reserved not in {,}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LITERAL} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.972. Support: 124.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.4, "max_conf": 0.9967391490936279, "max_support": 1753, "min_conf": 0.9278728365898132, "min_support": 124, "num_rules": 5}}
```
</details>
