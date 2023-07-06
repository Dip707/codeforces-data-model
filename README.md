# Codeforces Data Model
This project illustrates the functionalities of TypeDB and TypeQL working with the codeforces data model.  
All data used in this project was generated using the Codeforces [API](https://codeforces.com/apiHelp).  

## How to use this repo
Ensure that you have git installed in your system.  
To clone the repository, you can use the following clone command in your terminal.
```bash
$ git clone https://github.com/Yash-bhagwat/codeforces-data-model.git
```

We also need to run the typedb server. The download zip can be found [here](https://repo.vaticle.com/#browse/browse:artifact-snapshot:vaticle_typedb%2F07b9dfe04c786888a68f70b6f46dfdad1c9bb2e5). Please download the server required for your operating system (i.e. Windows, Mac or Linux).

Next, we extract our folder and run the following command in the "...\typedb-server-{your-version}-07b.." directory.  

```bash
$ ./typedb server --server.address=localhost:1729
```

Once the server is up and running the 'cargo run' command can be invoked at the root of our repo.

Currently this program supports 3 types of queries to choose from which are  
1) Get names of all coders with rating >= x  
2) Get IDs of problems with a particular tag
3) Get problem-name of problems with a particular tag with rating >= x
where paramters x and tag can be supplied by the user.

## How the commands were generated
The json data for the problems and the coders and the text file for the problem tags obtained from the CF Api was converted into TypeQL with the help of the dataloader.py file [link here](src/dataloader.py).


