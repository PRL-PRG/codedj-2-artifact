# Model report for file:///tmp/top-repos-quality-repos-yyo5obn_/whatsfordinner3.git HEAD 52f4c0e9e4995cd22746d0c8bb8c28c7a26f5342

### Dump

```json
{'created_at': '2021-09-02 01:18:01',
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
 'size': '18.2 kB',
 'tags': [],
 'uuid': 'f9f552a5-9e9c-487b-b938-5feb06ec61b4',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-yyo5obn_/whatsfordinner3.git 52f4c0e9e4995cd22746d0c8bb8c28c7a26f5342

# javascript
12 rules, avg.len. 5.9
## train
PPCR: 0.667891
### report
macro
{'f1-score': 0.4670519538673316,
 'precision': 0.4693810367001173,
 'recall': 0.46519775643704,
 'support': 6003}
micro
{'f1-score': 0.952023988005997,
 'precision': 0.952023988005997,
 'recall': 0.952023988005997,
 'support': 6003}
weighted
{'f1-score': 0.9445302431527716,
 'precision': 0.9374114090098249,
 'recall': 0.952023988005997,
 'support': 6003}
### report_full
macro
{'f1-score': 0.36983203395141706,
 'precision': 0.4693810367001173,
 'recall': 0.31615426174468964,
 'support': 8988}
micro
{'f1-score': 0.7624574744846909,
 'precision': 0.952023988005997,
 'recall': 0.6358477970627503,
 'support': 8988}
weighted
{'f1-score': 0.7307819306268198,
 'precision': 0.8805667563123666,
 'recall': 0.6358477970627503,
 'support': 8988}
## test
PPCR: 0.595070
### report
macro
{'f1-score': 0.48268949940614103,
 'precision': 0.47895973154362415,
 'recall': 0.48801258465312214,
 'support': 507}
micro
{'f1-score': 0.9526627218934911,
 'precision': 0.9526627218934911,
 'recall': 0.9526627218934911,
 'support': 507}
weighted
{'f1-score': 0.9457237876105367,
 'precision': 0.9413868039836032,
 'recall': 0.9526627218934911,
 'support': 507}
### report_full
macro
{'f1-score': 0.37028513533072804,
 'precision': 0.47895973154362415,
 'recall': 0.31321729888095867,
 'support': 852}
micro
{'f1-score': 0.7108167770419426,
 'precision': 0.9526627218934911,
 'recall': 0.5669014084507042,
 'support': 852}
weighted
{'f1-score': 0.685609095301291,
 'precision': 0.8921718603942822,
 'recall': 0.5669014084507042,
 'support': 852}
```

## javascript
### Summary
8 rules, avg.len. 5.5

| | |
|-|-|
|Min support|141|
|Max support|1573|
|Min confidence|0.9473933577537537|
|Max confidence|0.9981412887573242|

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
| 1 | `  -1.reserved not in {{}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 141.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type = JSXIdentifier<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = "<br>Confidence: 0.997. Support: 155.` |
| 3 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type not in {JSXIdentifier}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.992. Support: 185.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type = JSXAttribute<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = "<br>Confidence: 0.997. Support: 156.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 1573.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.internal_type = Identifier<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 395.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 1055.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 269.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.5, "max_conf": 0.9981412887573242, "max_support": 1573, "min_conf": 0.9473933577537537, "min_support": 141, "num_rules": 8}}
```
</details>
