use microjson::*;

#[test]
fn large_input_read() {
    let value = JSONValue::load(JSON_PAYLOAD);
    assert_eq!(
        value
            .iter_array()
            .unwrap()
            .nth(7)
            .unwrap()
            .get_key_value("_id")
            .unwrap()
            .read_string(),
        Ok("6136737d845794b21eddadc0")
    );
    assert_eq!(
        value
            .iter_array()
            .unwrap()
            .nth(7)
            .unwrap()
            .get_key_value("friends")
            .unwrap()
            .iter_array()
            .unwrap()
            .nth(2)
            .unwrap()
            .get_key_value("name")
            .unwrap()
            .read_string(),
        Ok("Rosario Curtis")
    );
}

const JSON_PAYLOAD : &str = "[
  {
    \"_id\": \"6136737d155df9328fdae6fa\",
    \"index\": 0,
    \"guid\": \"3dd3a16c-376c-4667-85fb-dba1d9153d5a\",
    \"isActive\": false,
    \"balance\": \"$1,605.58\",
    \"picture\": \"http://placehold.it/32x32\",
    \"age\": 29,
    \"eyeColor\": \"brown\",
    \"name\": \"Cash Mccray\",
    \"gender\": \"male\",
    \"company\": \"BOSTONIC\",
    \"email\": \"cashmccray@bostonic.com\",
    \"phone\": \"+1 (967) 462-3122\",
    \"address\": \"354 Degraw Street, Edinburg, Missouri, 547\",
    \"about\": \"Ad veniam ad esse ea ut magna reprehenderit veniam. Minim ex est elit enim nisi id exercitation non id ipsum magna. Cillum commodo tempor qui nostrud reprehenderit id consectetur consequat.\r\n\",
    \"registered\": \"2017-06-20T07:24:55 -01:00\",
    \"latitude\": 85.233704,
    \"longitude\": 13.95815,
    \"tags\": [
      \"aliquip\",
      \"aliqua\",
      \"ullamco\",
      \"qui\",
      \"elit\",
      \"labore\",
      \"reprehenderit\"
    ],
    \"friends\": [
      {
        \"id\": 0,
        \"name\": \"Oliver Jordan\"
      },
      {
        \"id\": 1,
        \"name\": \"Stout Meyers\"
      },
      {
        \"id\": 2,
        \"name\": \"Louella Myers\"
      }
    ],
    \"greeting\": \"Hello, Cash Mccray! You have 4 unread messages.\",
    \"favoriteFruit\": \"strawberry\"
  },
  {
    \"_id\": \"6136737dbcf6ee7047e05ea6\",
    \"index\": 1,
    \"guid\": \"f6987e4e-8c77-4120-9195-a2c5343f5373\",
    \"isActive\": false,
    \"balance\": \"$3,427.98\",
    \"picture\": \"http://placehold.it/32x32\",
    \"age\": 38,
    \"eyeColor\": \"brown\",
    \"name\": \"Stacy Cardenas\",
    \"gender\": \"female\",
    \"company\": \"KEENGEN\",
    \"email\": \"stacycardenas@keengen.com\",
    \"phone\": \"+1 (956) 479-3419\",
    \"address\": \"930 Llama Court, Summerset, Guam, 2791\",
    \"about\": \"Occaecat nisi labore amet ut minim ex Lorem commodo enim proident dolore laborum cillum ad. Minim do qui esse aliquip ut nostrud mollit consequat. Laborum officia mollit anim ad nulla pariatur irure magna consectetur duis officia. Eu quis non eiusmod velit veniam. Adipisicing eu dolore nulla commodo et adipisicing aliquip culpa enim amet dolor amet. Do reprehenderit consectetur labore enim velit id.\r\n\",
    \"registered\": \"2017-11-25T06:01:14 -00:00\",
    \"latitude\": 10.036853,
    \"longitude\": -49.255527,
    \"tags\": [
      \"adipisicing\",
      \"mollit\",
      \"aliquip\",
      \"officia\",
      \"exercitation\",
      \"consectetur\",
      \"ad\"
    ],
    \"friends\": [
      {
        \"id\": 0,
        \"name\": \"Winters Hawkins\"
      },
      {
        \"id\": 1,
        \"name\": \"Selma Waters\"
      },
      {
        \"id\": 2,
        \"name\": \"Earlene Santos\"
      }
    ],
    \"greeting\": \"Hello, Stacy Cardenas! You have 5 unread messages.\",
    \"favoriteFruit\": \"strawberry\"
  },
  {
    \"_id\": \"6136737d21f6eefd16e61520\",
    \"index\": 2,
    \"guid\": \"5f592c25-2721-4960-9c60-6c0a71ab0ede\",
    \"isActive\": true,
    \"balance\": \"$2,899.76\",
    \"picture\": \"http://placehold.it/32x32\",
    \"age\": 38,
    \"eyeColor\": \"brown\",
    \"name\": \"Roman Solis\",
    \"gender\": \"male\",
    \"company\": \"SIGNITY\",
    \"email\": \"romansolis@signity.com\",
    \"phone\": \"+1 (812) 537-3964\",
    \"address\": \"648 Merit Court, Waumandee, Indiana, 8393\",
    \"about\": \"Consectetur eu in exercitation pariatur amet nisi fugiat culpa irure dolor ad quis laborum. Fugiat laborum eu culpa in laboris aliquip est proident deserunt adipisicing consequat id laborum esse. In cupidatat cillum officia ex excepteur veniam elit aute ut tempor. Lorem exercitation ut id fugiat irure in consectetur cupidatat Lorem consequat qui. Sit ex incididunt et quis reprehenderit Lorem magna ipsum.\r\n\",
    \"registered\": \"2014-12-27T02:08:36 -00:00\",
    \"latitude\": 52.682357,
    \"longitude\": -55.960438,
    \"tags\": [
      \"duis\",
      \"cillum\",
      \"ut\",
      \"officia\",
      \"exercitation\",
      \"cupidatat\",
      \"enim\"
    ],
    \"friends\": [
      {
        \"id\": 0,
        \"name\": \"Battle Arnold\"
      },
      {
        \"id\": 1,
        \"name\": \"Rena Carey\"
      },
      {
        \"id\": 2,
        \"name\": \"Beach Snider\"
      }
    ],
    \"greeting\": \"Hello, Roman Solis! You have 3 unread messages.\",
    \"favoriteFruit\": \"strawberry\"
  },
  {
    \"_id\": \"6136737d65273f27f094f40b\",
    \"index\": 3,
    \"guid\": \"478d4f99-8c0d-43d5-9012-149d2a5fa084\",
    \"isActive\": false,
    \"balance\": \"$1,804.56\",
    \"picture\": \"http://placehold.it/32x32\",
    \"age\": 32,
    \"eyeColor\": \"green\",
    \"name\": \"Rivera Hale\",
    \"gender\": \"male\",
    \"company\": \"GAZAK\",
    \"email\": \"riverahale@gazak.com\",
    \"phone\": \"+1 (930) 491-3216\",
    \"address\": \"677 Diamond Street, Coloma, West Virginia, 2197\",
    \"about\": \"Aliquip cillum laboris laborum anim aliqua minim Lorem ea labore. Sit non nostrud ea Lorem velit nisi aliqua magna aliquip labore minim nulla laborum labore. Lorem minim tempor ad ut sint incididunt anim eiusmod nulla.\r\n\",
    \"registered\": \"2020-08-30T05:28:26 -01:00\",
    \"latitude\": -73.144359,
    \"longitude\": 53.215709,
    \"tags\": [
      \"dolore\",
      \"dolore\",
      \"est\",
      \"ad\",
      \"in\",
      \"consectetur\",
      \"cillum\"
    ],
    \"friends\": [
      {
        \"id\": 0,
        \"name\": \"Rosella Gallagher\"
      },
      {
        \"id\": 1,
        \"name\": \"Clemons Reilly\"
      },
      {
        \"id\": 2,
        \"name\": \"Moreno Nunez\"
      }
    ],
    \"greeting\": \"Hello, Rivera Hale! You have 1 unread messages.\",
    \"favoriteFruit\": \"banana\"
  },
  {
    \"_id\": \"6136737d29f735a79af0cc40\",
    \"index\": 4,
    \"guid\": \"4e5db036-b02c-40e8-9cde-5b237ca2e0ed\",
    \"isActive\": true,
    \"balance\": \"$1,501.93\",
    \"picture\": \"http://placehold.it/32x32\",
    \"age\": 34,
    \"eyeColor\": \"brown\",
    \"name\": \"Donaldson Mcintyre\",
    \"gender\": \"male\",
    \"company\": \"ARCHITAX\",
    \"email\": \"donaldsonmcintyre@architax.com\",
    \"phone\": \"+1 (905) 517-3690\",
    \"address\": \"208 Newton Street, Bison, Michigan, 9227\",
    \"about\": \"Laborum magna consequat ea non duis excepteur. Ipsum irure ullamco sunt deserunt sunt eiusmod dolor do aute reprehenderit dolor laboris. Pariatur cillum dolor voluptate eiusmod ad tempor adipisicing dolor ipsum. Pariatur ut occaecat occaecat ipsum id.\r\n\",
    \"registered\": \"2017-10-19T11:22:10 -01:00\",
    \"latitude\": -6.008692,
    \"longitude\": -17.349018,
    \"tags\": [
      \"magna\",
      \"quis\",
      \"est\",
      \"pariatur\",
      \"irure\",
      \"veniam\",
      \"do\"
    ],
    \"friends\": [
      {
        \"id\": 0,
        \"name\": \"Hoffman Jones\"
      },
      {
        \"id\": 1,
        \"name\": \"Lelia Terrell\"
      },
      {
        \"id\": 2,
        \"name\": \"Ann Hensley\"
      }
    ],
    \"greeting\": \"Hello, Donaldson Mcintyre! You have 8 unread messages.\",
    \"favoriteFruit\": \"banana\"
  },
  {
    \"_id\": \"6136737d4271de9ad7914096\",
    \"index\": 5,
    \"guid\": \"0122e0b0-fc7f-41b2-9f17-e3972503194c\",
    \"isActive\": true,
    \"balance\": \"$2,771.41\",
    \"picture\": \"http://placehold.it/32x32\",
    \"age\": 38,
    \"eyeColor\": \"green\",
    \"name\": \"Ladonna Compton\",
    \"gender\": \"female\",
    \"company\": \"OLUCORE\",
    \"email\": \"ladonnacompton@olucore.com\",
    \"phone\": \"+1 (938) 419-2099\",
    \"address\": \"902 Stockholm Street, Morgandale, Texas, 5356\",
    \"about\": \"Deserunt eu fugiat occaecat culpa ullamco commodo incididunt dolor proident exercitation. Excepteur anim exercitation nostrud magna. Voluptate quis Lorem occaecat nostrud officia laboris esse. Do dolore consectetur sint ipsum sunt enim labore ullamco id duis deserunt sint elit. Ullamco adipisicing esse aute ad mollit aute culpa eu ipsum culpa est Lorem et. Occaecat excepteur quis anim duis nulla sit nulla nostrud tempor aute exercitation aliqua ipsum.\r\n\",
    \"registered\": \"2019-05-21T05:29:13 -01:00\",
    \"latitude\": -8.531072,
    \"longitude\": -7.093529,
    \"tags\": [
      \"pariatur\",
      \"voluptate\",
      \"dolore\",
      \"esse\",
      \"eiusmod\",
      \"voluptate\",
      \"consequat\"
    ],
    \"friends\": [
      {
        \"id\": 0,
        \"name\": \"Reba Case\"
      },
      {
        \"id\": 1,
        \"name\": \"Conway Faulkner\"
      },
      {
        \"id\": 2,
        \"name\": \"Gena Merritt\"
      }
    ],
    \"greeting\": \"Hello, Ladonna Compton! You have 3 unread messages.\",
    \"favoriteFruit\": \"apple\"
  },
  {
    \"_id\": \"6136737d5793850fd355ab79\",
    \"index\": 6,
    \"guid\": \"19fca6e2-d05d-4c66-8719-bd134d4f7168\",
    \"isActive\": false,
    \"balance\": \"$2,477.64\",
    \"picture\": \"http://placehold.it/32x32\",
    \"age\": 25,
    \"eyeColor\": \"green\",
    \"name\": \"Dominique Dixon\",
    \"gender\": \"female\",
    \"company\": \"SNIPS\",
    \"email\": \"dominiquedixon@snips.com\",
    \"phone\": \"+1 (915) 551-3738\",
    \"address\": \"220 Tapscott Street, Johnsonburg, New York, 249\",
    \"about\": \"Qui consequat amet adipisicing irure ex enim sint fugiat eu dolor ea non magna proident. Magna eiusmod id ullamco ad tempor cillum excepteur cupidatat fugiat fugiat anim. Ullamco elit et Lorem ut. Quis eiusmod anim adipisicing aliquip laboris ullamco ipsum officia.\r\n\",
    \"registered\": \"2018-07-19T11:53:46 -01:00\",
    \"latitude\": -57.166103,
    \"longitude\": 73.98064,
    \"tags\": [
      \"voluptate\",
      \"est\",
      \"id\",
      \"dolor\",
      \"consectetur\",
      \"esse\",
      \"cillum\"
    ],
    \"friends\": [
      {
        \"id\": 0,
        \"name\": \"Briggs Ortiz\"
      },
      {
        \"id\": 1,
        \"name\": \"Beulah Fletcher\"
      },
      {
        \"id\": 2,
        \"name\": \"Burns George\"
      }
    ],
    \"greeting\": \"Hello, Dominique Dixon! You have 9 unread messages.\",
    \"favoriteFruit\": \"banana\"
  },
  {
    \"_id\": \"6136737d845794b21eddadc0\",
    \"index\": 7,
    \"guid\": \"1d7738ca-c505-4d95-87b4-f2c2d013dbf2\",
    \"isActive\": true,
    \"balance\": \"$2,755.38\",
    \"picture\": \"http://placehold.it/32x32\",
    \"age\": 33,
    \"eyeColor\": \"brown\",
    \"name\": \"Cunningham Pugh\",
    \"gender\": \"male\",
    \"company\": \"ACCUPHARM\",
    \"email\": \"cunninghampugh@accupharm.com\",
    \"phone\": \"+1 (905) 400-2716\",
    \"address\": \"498 Albee Square, Forestburg, Federated States Of Micronesia, 7103\",
    \"about\": \"Aliquip excepteur ea ad consectetur ea non sunt duis. Pariatur et incididunt laborum ex occaecat amet velit. Fugiat id excepteur sunt laborum cillum minim ut. Est duis cupidatat ea laboris dolore tempor ut nulla ipsum in proident. Fugiat ad cillum est nulla irure sit nisi laborum eiusmod qui est aliqua. Non sit culpa adipisicing commodo aute fugiat esse consectetur laboris sunt ad eiusmod.\r\n\",
    \"registered\": \"2018-12-01T06:49:38 -00:00\",
    \"latitude\": -29.710314,
    \"longitude\": -161.367617,
    \"tags\": [
      \"qui\",
      \"reprehenderit\",
      \"ea\",
      \"sit\",
      \"tempor\",
      \"do\",
      \"sunt\"
    ],
    \"friends\": [
      {
        \"id\": 0,
        \"name\": \"Amalia Holder\"
      },
      {
        \"id\": 1,
        \"name\": \"Burnett Turner\"
      },
      {
        \"id\": 2,
        \"name\": \"Rosario Curtis\"
      }
    ],
    \"greeting\": \"Hello, Cunningham Pugh! You have 1 unread messages.\",
    \"favoriteFruit\": \"apple\"
  },
  {
    \"_id\": \"6136737d04fbbbbe46620def\",
    \"index\": 8,
    \"guid\": \"13d4e6fa-cf6b-4c81-95fc-8e1ddfad3686\",
    \"isActive\": false,
    \"balance\": \"$3,628.27\",
    \"picture\": \"http://placehold.it/32x32\",
    \"age\": 20,
    \"eyeColor\": \"green\",
    \"name\": \"Judith Padilla\",
    \"gender\": \"female\",
    \"company\": \"TERRAGO\",
    \"email\": \"judithpadilla@terrago.com\",
    \"phone\": \"+1 (886) 587-3828\",
    \"address\": \"382 Elliott Place, Deputy, Marshall Islands, 3530\",
    \"about\": \"Consectetur ad laboris occaecat anim nulla ipsum exercitation Lorem eiusmod dolore culpa. Mollit elit culpa nostrud labore velit excepteur eiusmod consectetur incididunt pariatur sint reprehenderit commodo incididunt. Labore elit reprehenderit labore ea labore ad magna laborum eiusmod non in voluptate esse. In excepteur elit adipisicing id laborum laboris qui velit eiusmod.\r\n\",
    \"registered\": \"2014-03-27T08:37:54 -00:00\",
    \"latitude\": -56.506466,
    \"longitude\": -157.849193,
    \"tags\": [
      \"velit\",
      \"minim\",
      \"tempor\",
      \"non\",
      \"nostrud\",
      \"ipsum\",
      \"ex\"
    ],
    \"friends\": [
      {
        \"id\": 0,
        \"name\": \"Catalina Pollard\"
      },
      {
        \"id\": 1,
        \"name\": \"Karyn Richards\"
      },
      {
        \"id\": 2,
        \"name\": \"Rhoda Harris\"
      }
    ],
    \"greeting\": \"Hello, Judith Padilla! You have 3 unread messages.\",
    \"favoriteFruit\": \"strawberry\"
  },
  {
    \"_id\": \"6136737dff0b1ce129346b81\",
    \"index\": 9,
    \"guid\": \"ff440fcc-ee4c-46f8-acef-ca61061fe0b4\",
    \"isActive\": false,
    \"balance\": \"$1,831.66\",
    \"picture\": \"http://placehold.it/32x32\",
    \"age\": 38,
    \"eyeColor\": \"blue\",
    \"name\": \"Krystal Thomas\",
    \"gender\": \"female\",
    \"company\": \"ELPRO\",
    \"email\": \"krystalthomas@elpro.com\",
    \"phone\": \"+1 (891) 561-3938\",
    \"address\": \"394 Ryerson Street, Thornport, Kansas, 2002\",
    \"about\": \"Est amet nostrud culpa cupidatat. Ea amet sit sit ex ad ut consectetur ullamco nostrud enim amet reprehenderit. Ut sunt reprehenderit cillum pariatur. Ex reprehenderit exercitation laborum ex.\r\n\",
    \"registered\": \"2014-04-04T06:01:40 -01:00\",
    \"latitude\": 65.437772,
    \"longitude\": -68.878508,
    \"tags\": [
      \"magna\",
      \"pariatur\",
      \"labore\",
      \"non\",
      \"ea\",
      \"qui\",
      \"consectetur\"
    ],
    \"friends\": [
      {
        \"id\": 0,
        \"name\": \"Spears Leblanc\"
      },
      {
        \"id\": 1,
        \"name\": \"Cleveland Shaffer\"
      },
      {
        \"id\": 2,
        \"name\": \"Sykes Walker\"
      }
    ],
    \"greeting\": \"Hello, Krystal Thomas! You have 3 unread messages.\",
    \"favoriteFruit\": \"strawberry\"
  },
  {
    \"_id\": \"6136737d523a42285bd2d526\",
    \"index\": 10,
    \"guid\": \"2ab868d5-eaa0-421f-8b01-b95e426dd774\",
    \"isActive\": true,
    \"balance\": \"$1,636.26\",
    \"picture\": \"http://placehold.it/32x32\",
    \"age\": 34,
    \"eyeColor\": \"green\",
    \"name\": \"Raymond Valencia\",
    \"gender\": \"male\",
    \"company\": \"EXTRAWEAR\",
    \"email\": \"raymondvalencia@extrawear.com\",
    \"phone\": \"+1 (945) 543-3179\",
    \"address\": \"200 Seagate Terrace, Delco, Virgin Islands, 1955\",
    \"about\": \"Voluptate nostrud occaecat aliquip labore. Aliqua voluptate irure nulla eiusmod velit duis commodo officia ullamco eiusmod sunt proident officia. Ex amet et incididunt reprehenderit voluptate ipsum eu id ullamco pariatur esse et aliquip commodo. Ullamco do aute esse qui sunt id eiusmod sunt. Anim dolor commodo aliquip nisi enim labore.\r\n\",
    \"registered\": \"2019-10-11T05:11:05 -01:00\",
    \"latitude\": 16.392988,
    \"longitude\": 96.596231,
    \"tags\": [
      \"aliqua\",
      \"quis\",
      \"ea\",
      \"consectetur\",
      \"amet\",
      \"ex\",
      \"exercitation\"
    ],
    \"friends\": [
      {
        \"id\": 0,
        \"name\": \"Clark Blackburn\"
      },
      {
        \"id\": 1,
        \"name\": \"Ola Morrow\"
      },
      {
        \"id\": 2,
        \"name\": \"Aguilar Camacho\"
      }
    ],
    \"greeting\": \"Hello, Raymond Valencia! You have 4 unread messages.\",
    \"favoriteFruit\": \"apple\"
  }
]";
