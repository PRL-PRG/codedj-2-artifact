# Model report for file:///tmp/top-repos-quality-repos-uxe5t4qv/website.git HEAD f2e7abc3af248dc6ef69b0dc0868643db2adb8e1

### Dump

```json
{'created_at': '2021-09-02 06:01:01',
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
 'size': '16.6 kB',
 'tags': [],
 'uuid': '28ab1dbc-fd05-40dd-8644-9a19f315f03a',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-uxe5t4qv/website.git f2e7abc3af248dc6ef69b0dc0868643db2adb8e1

# javascript
13 rules, avg.len. 5.0
## train
PPCR: 0.641031
### report
macro
{'f1-score': 0.5724936184198424,
 'precision': 0.5853677485232268,
 'recall': 0.5613108907644229,
 'support': 6318}
micro
{'f1-score': 0.9314656536878759,
 'precision': 0.9314656536878759,
 'recall': 0.9314656536878759,
 'support': 6318}
weighted
{'f1-score': 0.9238095019420562,
 'precision': 0.9172393584108459,
 'recall': 0.9314656536878759,
 'support': 6318}
### report_full
macro
{'f1-score': 0.40363505331333294,
 'precision': 0.5853677485232268,
 'recall': 0.3250635860379554,
 'support': 9856}
micro
{'f1-score': 0.7277111413379499,
 'precision': 0.9314656536878759,
 'recall': 0.5970982142857143,
 'support': 9856}
weighted
{'f1-score': 0.6750721155674908,
 'precision': 0.8222653397509704,
 'recall': 0.5970982142857143,
 'support': 9856}
## test
PPCR: 0.658850
### report
macro
{'f1-score': 0.5789037238928885,
 'precision': 0.5998741102529154,
 'recall': 0.5659983221290656,
 'support': 1180}
micro
{'f1-score': 0.9398305084745763,
 'precision': 0.9398305084745763,
 'recall': 0.9398305084745763,
 'support': 1180}
weighted
{'f1-score': 0.932054975429912,
 'precision': 0.9265373084139976,
 'recall': 0.9398305084745763,
 'support': 1180}
### report_full
macro
{'f1-score': 0.42211233451665225,
 'precision': 0.5998741102529154,
 'recall': 0.3367823324502429,
 'support': 1791}
micro
{'f1-score': 0.7465499831706496,
 'precision': 0.9398305084745763,
 'recall': 0.6192071468453378,
 'support': 1791}
weighted
{'f1-score': 0.6924073979274223,
 'precision': 0.8133710435496513,
 'recall': 0.6192071468453378,
 'support': 1791}
```

## javascript
### Summary
8 rules, avg.len. 5.2

| | |
|-|-|
|Min support|97|
|Max support|2423|
|Min confidence|0.9275691509246826|
|Max confidence|0.9986737370491028|

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
| 1 | `  -1.label in {<space>}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.996. Support: 113.` |
| 2 | `  +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 419.` |
| 3 | `  +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {IDENTIFIER} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 377.` |
| 4 | `  +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXAttribute<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.985. Support: 97.` |
| 5 | `  +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXAttribute<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 196.` |
| 6 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXAttribute, JSXElement}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 135.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles in {EXPRESSION}<br>	∧ -4.roles in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 117.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.928. Support: 2423.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.25, "max_conf": 0.9986737370491028, "max_support": 2423, "min_conf": 0.9275691509246826, "min_support": 97, "num_rules": 8}}
```
</details>
