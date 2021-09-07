# Model report for file:///tmp/top-repos-quality-repos-oknrmilz/telescope.git HEAD e82daf606bf4ac64c445b5bd58f80784dd16f97a

### Dump

```json
{'created_at': '2021-09-02 06:11:34',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.19.0-12-amd64-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '15.9 kB',
 'tags': [],
 'uuid': '6830eb14-3fab-416c-8f58-e068ed796d5b',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-oknrmilz/telescope.git e82daf606bf4ac64c445b5bd58f80784dd16f97a

# javascript
12 rules, avg.len. 5.9
## train
PPCR: 0.834606
### report
macro
{'f1-score': 0.6717983421185308,
 'precision': 0.6844456571876953,
 'recall': 0.6618544107248593,
 'support': 3825}
micro
{'f1-score': 0.9317647058823529,
 'precision': 0.9317647058823529,
 'recall': 0.9317647058823529,
 'support': 3825}
weighted
{'f1-score': 0.9236760266470863,
 'precision': 0.9180845070939868,
 'recall': 0.9317647058823529,
 'support': 3825}
### report_full
macro
{'f1-score': 0.6391670315823929,
 'precision': 0.6844456571876953,
 'recall': 0.6027911272415751,
 'support': 4583}
micro
{'f1-score': 0.8477640342530923,
 'precision': 0.9317647058823529,
 'recall': 0.7776565568404975,
 'support': 4583}
weighted
{'f1-score': 0.8140911552968109,
 'precision': 0.8579110542955459,
 'recall': 0.7776565568404975,
 'support': 4583}
## test
PPCR: 0.786976
### report
macro
{'f1-score': 0.7003532147725737,
 'precision': 0.7014995245409992,
 'recall': 0.6994704385938378,
 'support': 713}
micro
{'f1-score': 0.967741935483871,
 'precision': 0.967741935483871,
 'recall': 0.967741935483871,
 'support': 713}
weighted
{'f1-score': 0.9671781787072886,
 'precision': 0.9672046242376061,
 'recall': 0.967741935483871,
 'support': 713}
### report_full
macro
{'f1-score': 0.6678823110724219,
 'precision': 0.7014995245409992,
 'recall': 0.6412823816785401,
 'support': 906}
micro
{'f1-score': 0.8523780111179741,
 'precision': 0.967741935483871,
 'recall': 0.7615894039735099,
 'support': 906}
weighted
{'f1-score': 0.8162307348060621,
 'precision': 0.8859900826147356,
 'recall': 0.7615894039735099,
 'support': 906}
```

## javascript
### Summary
10 rules, avg.len. 4.7

| | |
|-|-|
|Min support|101|
|Max support|760|
|Min confidence|0.9286971688270569|
|Max confidence|0.9969512224197388|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.939. Support: 760.` |
| 2 | `  -1.internal_type = Identifier<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 127.` |
| 3 | `  -1.internal_type = Identifier<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.929. Support: 568.` |
| 4 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = {<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.989. Support: 138.` |
| 5 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {{}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.997. Support: 164.` |
| 6 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.981. Support: 129.` |
| 7 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 191.` |
| 8 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.996. Support: 122.` |
| 9 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 131.` |
| 10 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 101.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.7, "max_conf": 0.9969512224197388, "max_support": 760, "min_conf": 0.9286971688270569, "min_support": 101, "num_rules": 10}}
```
</details>
