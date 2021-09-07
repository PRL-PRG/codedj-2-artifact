# Model report for file:///tmp/top-repos-quality-repos-lt4zbk3p/donde_front_end.git HEAD e7fdae24d6bbaeb1f8463be9c336946fc24365a9

### Dump

```json
{'created_at': '2021-09-02 05:46:22',
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
 'size': '17.2 kB',
 'tags': [],
 'uuid': 'fdc0ce52-6f39-44f9-a76d-d5766afc7722',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-lt4zbk3p/donde_front_end.git e7fdae24d6bbaeb1f8463be9c336946fc24365a9

# javascript
21 rules, avg.len. 7.3
## train
PPCR: 0.780032
### report
macro
{'f1-score': 0.662891791832313,
 'precision': 0.692443873459389,
 'recall': 0.6477724685089892,
 'support': 15844}
micro
{'f1-score': 0.9441428932087856,
 'precision': 0.9441428932087856,
 'recall': 0.9441428932087856,
 'support': 15844}
weighted
{'f1-score': 0.9401739496438734,
 'precision': 0.9383552864427882,
 'recall': 0.9441428932087856,
 'support': 15844}
### report_full
macro
{'f1-score': 0.5518922988846011,
 'precision': 0.692443873459389,
 'recall': 0.4721765099614495,
 'support': 20312}
micro
{'f1-score': 0.8274698528598297,
 'precision': 0.9441428932087856,
 'recall': 0.7364612051988972,
 'support': 20312}
weighted
{'f1-score': 0.7966122549633461,
 'precision': 0.8804095750354769,
 'recall': 0.7364612051988972,
 'support': 20312}
## test
PPCR: 0.762634
### report
macro
{'f1-score': 0.6223703264517021,
 'precision': 0.6717447134936372,
 'recall': 0.598364823723623,
 'support': 4331}
micro
{'f1-score': 0.9473562687601016,
 'precision': 0.9473562687601016,
 'recall': 0.9473562687601016,
 'support': 4331}
weighted
{'f1-score': 0.943219884606149,
 'precision': 0.9419614737991499,
 'recall': 0.9473562687601016,
 'support': 4331}
### report_full
macro
{'f1-score': 0.4874271442894753,
 'precision': 0.6717447134936372,
 'recall': 0.41173770341336163,
 'support': 5679}
micro
{'f1-score': 0.8197802197802198,
 'precision': 0.9473562687601016,
 'recall': 0.7224863532312027,
 'support': 5679}
weighted
{'f1-score': 0.7840098021248014,
 'precision': 0.8779327772677047,
 'recall': 0.7224863532312027,
 'support': 5679}
```

## javascript
### Summary
12 rules, avg.len. 7.1

| | |
|-|-|
|Min support|109|
|Max support|7388|
|Min confidence|0.9200819730758667|
|Max confidence|0.9976744055747986|

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
| 1 | `  -2.reserved = from<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.997. Support: 148.` |
| 2 | `  -2.reserved not in {from}<br>	∧ +1.reserved = ><br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 215.` |
| 3 | `  -1.reserved not in {{}<br>	∧ -2.reserved not in {from}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 174.` |
| 4 | `  -1.reserved not in {{}<br>	∧ -2.reserved not in {from}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.937. Support: 135.` |
| 5 | `  -1.reserved not in {{}<br>	∧ -2.reserved not in {from}<br>	∧ +1.reserved not in {), ,, >}<br>	∧ +2.reserved not in {)}<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 842.` |
| 6 | `  -1.internal_type = CommentLine<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.930. Support: 463.` |
| 7 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {:, =}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.920. Support: 219.` |
| 8 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 7<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {MODULE} and not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 211.` |
| 9 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {import}<br>	∧ +1.length ≤ 6<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {MODULE} and not in {DECLARATION}<br>⇒ y = ⏎⏎<br>Confidence: 0.940. Support: 109.` |
| 10 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {OPERATOR} and not in {DECLARATION, MODULE, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 114.` |
| 11 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, MODULE, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.920. Support: 244.` |
| 12 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -4.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {DECLARATION, MODULE, OPERATOR, STATEMENT, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 7388.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.083333333333333, "max_conf": 0.9976744055747986, "max_support": 7388, "min_conf": 0.9200819730758667, "min_support": 109, "num_rules": 12}}
```
</details>
