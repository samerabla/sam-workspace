use dioxus::prelude::*;

// const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
// const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

fn App() -> Element {
    rsx! {
        // document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        h1 { style: "color:white; background:black; padding:10px;", "Hello Ahmad" }
        p { "hello world" }
        img {
            class: "img",
            src: "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAAAQABAAD/2wCEAAkGBxATEhUTExIWFhUVGRcZFxcXGBgYGBoYFxUWFxcWHxsZHSggGBsnGxcVIzEjJSorLi4uFyAzODctNyotLisBCgoKDg0OGxAQGyslICYrKy43NzUtLTUvKzYtLS0tLS8wLTUtLS0tLS0tKy0tLy0tLS0vLi8tLS0tLS0tLS0tNf/AABEIAQMAwgMBIgACEQEDEQH/xAAcAAEAAgMBAQEAAAAAAAAAAAAABgcCBAUDCAH/xABBEAACAQIEAwUGAwUGBgMAAAABAgADEQQSITEFBkEiUWFxgQcTMkKRoVJisRQjgsHRcpKisuHwFjNDRFOTFcPx/8QAGgEBAAMBAQEAAAAAAAAAAAAAAAMEBQIBBv/EACsRAAMAAgIBAgUEAgMAAAAAAAABAgMRBCExEkETMlFhgQUicfCh0TOxwf/aAAwDAQACEQMRAD8Au+IiAIiIAiIgCIiAIiIAic7imNIpBqbC7lQjbjXtX8RlBmL8ZQU1cC7NeyX1zA2YE9Ap0J+lyQDFWbHLab1pb/B7pnTicPhnEiHf31QAFVYE2VRYkMBfpqm5O83sPxWm7hEzG9zfKVGm57ViRqNQLaic4+RjySqT8+PuGmjeiIk54IiIAiIgCIiAIiIBlERAMYiIAiIgCIiAIiIAms2LQs1O/bAvlOhII3H4h5TZnC5ow4Ko4IFQEAA7MNyD3ZdWDdLH8Ujy24h0vY9S29HIwav7ukNMgXNbqGKAG3gbsfA999NvD0kzM1gCRqe8gWW/fPOhYKoXYAW1J0tbc6npOXV5lwSmxrrcb2DH9BPjXeTJe0nWv72WNaOv7tdzuPh79d9ek8GV0qCoHsNMy3C9kEsRmsSLnLfbRbTnpzLgj/3Cetx+om6+LU086NmQ7MpDDuuOh1iHlw0qSaf8f7PUt9HWw/Ma5Xq1slKhTGtVnsua/wAIuBm638bAXN7chOb8TjHNPh1C6jRsRXutNfJRqxt032uLayKY/hVLEV6f7ZWWng6XbARcpz63zOTamh6kDvvbQzq8T5+pUlXDcMoq3yq2Ulf4EHaqE6m58+1Pq+Nl9eNNshuPTWjdxnLPH6jZv/mFXuVMOqqPuSfWcqvy7zVTN6fEadQDoSoJ9GoEfebeA5O4jilL43HYikW2Sm9mHiQP3aeQX1G0wqezDFqP3PGcUndmNRv8tZZZOD25Yx3Mv7QKWJw9I0vmqvlXS+6tSY5jv2cvmRLFkI5V5GxFCqKuK4jiMSV+Cn7yqtLzZTUbOe4beelpvPUeCIiegREQDKIiAYxEQBERAEREAREQBIjzph6mV6jOQNKdJE+I37TknpoGNh/4113El00uMUQ1GppchKmU9xNNluO42JHqZxkhXPpZ7L09lWYLmI/tzUVI9zQoPTH5qyhXdr+SEehm7V5JwdLDpWqvVYsilhnFgSoJtlQFtTYd95F/+FqtJGq0qxJV/mQh2Z1W4IW97mpY7bHabTcz4nF01w7rTU0h8zNSc5SbMSbi/ZB8x0ItKPFwxj3S8aX/AL/3sltPaR60OXcI6M4q1Qi6ZjkNzvpYG+4+s8qPGGw+DqohzNRq02Cm1zTqOc6m2lyFqeWaaePqYqiMuRWJItTUszXKgLcLYLcWA0A+5jg3AnxK/tD1WTMaashp9GqmmLNexsTU7zfeSciIyRt+E0/7+DyVSrR2eH8kHijiscSq4ZSCiqrNUIZFb5jkTci9ibAgyzOAcsYTCD9zT7RFjUbtOfDMdh4LYeE1+ScAtPDUmFxnpUQV0/6alQdPmK5QenZkgljFjUSkcXW3sRESU5EREAREQBERAMoiIBjERAEREAREQBESNc9c0DA0QVAatUJFNTsLfE57wLjTqSBpqQBI6jhRckADck2EhnNHtDwtC9OjbEVOuU/u183F7nwW/jaVHxLiNfENmr1WqH8xuB5LsvoBNWcOj3RYPKXHRWDrdVdApZWOUHOX7atqbaWsbns+pkGE4bSprlVBbuIBtcAWGmi6DTbSUxw7jr4PGirqUKBKi6G6E3uAdCwIuL91usuXhGNpVh70Vi1FlXI1NQ3aBbOGGUkH4dLabaSq4c9J9MtY7lrbXaPephlYMrC+e4a9rkHpr02+k4XM3FKWEoC5GXOiIg1bMTmHWwsQWOmtidzO9iKVD3TJSerexAYgKEPVjop03t5X0lS858wrjMWlOkxahRLMDplZ9sy6fCDYA9dTsZ58Nv8Aa30e3a1vXZ0+C85Y/DCyVcy/gqDOov3agr5AgeEkWD9q+IFve4em465GZD9DmvK9n4rXv4afYH+cs7ZULu5T58oYw5CppVCeypNwQdtRoD085Lp8y06roQyGxBuDsRcWOvcev9ZavBvanQKgYqm1MjKM6HOp07TEWBXbYA76Tqa+oa+hYsTywuJSoi1KbB0YXVlNwQeoInrOzwREQBERAMoiIBjERAEREARE88TiEpqz1GVEUXZmICgDqSdAIBmxtqZQvPHMQx2JFRARSRQiZtzqSWsNr3HXYCdjnb2ivXvQwyulLNrVVrNUXawGmRTvvci22okED/lP2/rOKZ6jOJitQHz7jofvPPCVcy5uhJt5A2H6Tk9NbieDL2sLk6fe4Gu27fadDgvCuN4Vi1ChVUHU/AyN/aF7bdd9Ok861UKLnYes1OA4Z6+KSlQZwpOZlDFQEXtPou4t0t1AnL8Hs+SUc04XitalSp0Ud6dSij1xTygNWqZnYG5zW1Gm05XBuUMeoJOGcMTaxKCwHm3+7S0Xx3uWzVAVpudyAApAVVGr3tYG5IG3TaaeE544a65v2lF8Kl0P0Ya+khV0l0izWOW+2QzE8p48KXyIqqCz3dc2UC5tlzAnzttOItLc3Ovl3W7pL+Nc/wBKrnoYdGZWVlasTlABFjlUi7XFxrbv1tInnHePrJYdPyQZFKf7TH3f5j9f9J+e6G/XvOv/AOTzfGoDYXY9ygn/AEmaO5+Ww8Tr9B/WdnB6h6mXLnYKCSFBYJc7nJe1/HeXj7OePvi8L+8HbokU2bXt2RSH16m+viD3yjgdh1Ow6k9wHWXt7PuCNhcGquLVKhNSoOoLAAKfEKFB8bzqTxkliInZ4IiIBlERAMYiIAiIgCQP2xYh1wlNB8NSsofxCo7gf3lB9JPJxubuBjGYWpQuFY9qmxFwtRdVJ8Oh8CZ4wfPsTdxfKXG6RIbAs1utPK4Plla/2E1G4VxQb8OxPpRqn9FMj0z0wdARYgEeM81oACykqPCx/wAwM88S+JpW97hK9O+2enUQeXaQTV/+aTqCPUQDfCEfOT5hf5ASe+z/AINUpmpVrIFc2RBcE5SA5Om17oLdCplb0+MUrglWIBBIFtRfUb915OqntRpMtqOHqe9NwucpkB6ElWJI62t0keRU1pE2Fynts4XPvEqzFsOpZkFSo7sGJzXJFNLdEWlkuOrXPS5hIU3tY37usk9esosWYaqhJNgSxRSx8y1z6zpcK4S9ax+FDY362Ph09Yq5xzujxRWWv2oiuH4LVqMFAGupJ1Cjx8fDrLh9lHCESsco7NKmRtu1RhZj42R/rPDD0KdGnlUWUfUnvPeTJl7PSGw7VbavUYfwpYKPLc/xGVsGZ58v2XZbzYFgxfdnex3DqFZctWklRd7OqsL9+o3nPXlHho/7LD/+pD+onaiaRnGrhOGYel/y6NNP7CKv6CbURAEREAREQDKIiAYxEQBERAEREAREQCKe0mjfCq34Kqn6q6fq4lY1Uvra5Hf1HUf7/rLQ9ovEUp4RqZGZ6vwLf8DK5fyFh6kDxFZAzL5vWRNfQ1OH/wAen9Typ4Sg9syLY9Sim3mCJrc4cCT3KPTsGQhRYWBUg6abDabbKQbj1Hf/AK/78tXjePthmvqoZNOo7QHXzlfE6+JOn7k+VT8Ok17ELPDajGxGUEbmx6W9df0lvYLL7tSBYFQ31AMrwYpGVCGFgp8LfvH3koXiBNCkq9aaepyL9FH++6WOdLpT+St+n0pdfg9eJ4v3hyD4evl3+uw9TLL9nq2wS+L1P85H8pViJbzO58ZYvs04sKlD3BsHp3YdMyOxYHzBNj/D3zzg6V6+x1zduN/cmURE1TLEREAREQBERAMoiIBjERAEREAREQBERAKl50xbVMZVvtTIpqO4BQT9WZj9JwKIsLd2g8un209JZPO3LAqB8TTIWoqkup2cKu/g9hvsbAHvFYYHH0qy5qbBh3bEeY3EyOTjubbfhmvxskOEl5RtTkc1UwcM57iv+caTrzl8y0i2Ge3SzeikE/YfaQ4vnX8kuXuK/hkFIJG40sPrnb+Rk75drF6CuR2jcE94QlR5aDbzkGHwnwN/Tb9T95NuWK6th1C7pcMPG5N/W95e5fyfkz+E/wB7/g6pE6vKmJNLF0COrCmR3rU7NvK5Vv4RI1xPjNKjoxu/4F39fwjzk49irDEU6+JqUxnWrkptY2C+7UkC+l7kgka9PCV+NhuqVeEWuRmiZc+WWdERNcyRERAEREAREQDKIiAYxEQBERAEREARE4nNvM9DAUlq1gzZnCKiAF2JBJsCQLBQSdekAc512XB1QvxOPdqNtXNjr07OY+kpHFciW7dGrkboovZfBWvm+pljcQ5ww+ORVw5eyHNUDoyEG1kXUWb5joTbKvfNG0yeZyanJqX4RrcPjReLdryyArQ4lS0bJUH5jY/Ww+94xeOrmk6nC1AxVgMpVxci3Q3+0muIr5PiF1JtfuPcfA9/p3Txq4eg4uGC+KkD6g6fUSss++3KLD4+ulTKiXC1df3b3GhGU31I6SQ8vcBx5VsmWir2u7fFYX+EC/eddPOSipToq9rmoCCSGsF0yjoBfyOkft1QqgZtSALDS5sLyzfKqlpJFbHxJh7bZp4HlLCUiDWY16hPwnYsfyDVv4iRJxwDir4Uk/8ASPxUlyALZbBgTazaKDrltfS4uY3hagU2Rc9Q9egHcO4bXJtfTwE3RStZqzgt8q/KD4L8zeNvKVvjZFXq33/fYtfAxufTrr++5ZfCOPUMQbISGyhsrAqbHe1/isdDa+46EX6kqpKj3DJdCpBVvmBHUD6jXvII1kg4TzNVp3FbNVUm4bsh17xawDDruDqd9BNDDz4rq+n/AIKGbgVPcdr/ACTWJq8P4jSrLmpOGA3GzKe5lOqnzE2peT32ig1rpiIiengiIgGUREAxiIgCIiAIiIAlIe2Higq41aStdcOmUjoKjnM/mcoQeGo75c3E8YtGjVrNotJHdvJFLH7CfMHAqtTFYlEq3Y1WZqjDRtmdm7tT18ZzVKVtnsy6ale5M+TuF8RagWoYWmUqHMtTEMchWwAy0wQTe18zAggi03ccvFqGtbh+HqUxqWpU0uPI0mFRPPLadP3S2AIuAABmu1gBYDXpNvCY2rT+Bzb8JuV+nT0tM5c/E33JovgZVPVHD4dxSjiFYre1v3lNzdkubXzWHvKd9M2hU/ENQZpVaZVip+U7943B+n3vOjzVhqSsmPojLZguITvDFUqA202dLd4JPygDm4LC4jEVMtFDUcC72tYa6HtMANb7n5us5z4FtVj8M64+d6c5PKNWoblgN7BR66k+G4+kzpkXvffQtbf8qjr5D7mdM8k8Ty3NE6kkqKlEbm5va9xrawvtNCvhsTRPbUoRp2kZD6M3Zb00kVYqldomnLNPpnSwtCpbQe6U7neo3jrt67dwmynuaeuZQTuzNdj6k39JwM9z2mb+I2H20P1M38Pi8myJ6Cx+olapZZmkdNcWp2zHyRrf3rW+89Ecn5SPPL/ImaacUXqpH3mwmNpn5h66frI2vsSKl9TZo1GRg6Eqy7MN/LxHgdJKOFc1XbLXAW+1RbhfJgScvncjvt1iikHafsmw8m8T68fQizcaMq78/UtKJDOXOOmmVpVTemdFY7oToFP5PH5fL4ZnNzDmnLPqkws2GsVemhERJSIyiIgGMREAREQBERANPjPDqeJoVaFQkJVRkYqbHKwsbHpKf4RyxhsLiKrUK7V0sqqzKAR8zC40f5O0ABLC554yUT9mpa1awsx/8dJjlap/atmCjvF+kieHoqgsosLk/wB4kn9Zm8/OlPw0aXAwbr4j9j0iJp47GBRZT2v0/wBZkJbNdvRzuL0q2JLYagCzVFKhRbVlOjakCy5nO4lq8n8GOFwyU2tnN2cj8R2F+tlCrf8ALOL7PuXmpA4qoe1VXsLbVUYqbknqwVDbp66TSfQcbG4xrZ89ycivI2hPx1BFiAR3HUT9iWSucPH8pYKre9EIT1p9j7Dsn1Bkaxvs3I1oV7dyutv8S6f4JYMSKsMV5RLOa58Mp3G8sY+lfNQLgfNT7X6an1Czk+8F7G6t3NoZfE1sdhKLqfe00dQCTnUMLDzErVwZfyssTzaXlFKKxGxI8psJjqg6385xPc9rsllVhcdprBdMoIvo2oBPW3QmblCiQO0xJ7+0PsSZnVCRpTbOsOJ3FmQG/jLU5dru+FoO/wATU0JvubqNfWU/hsM1V0pLvUZUHhmNs3oLn0l3UaYVQqiwUAAdwAsBL3AjXqZR5979KM4iJomcZREQDGIiAIiIAnjjMSlKm9RzZEVmY9yqCSfoJ7SL88Y0ZEoA6uQ7D8iEH7vlHiA04yWol0/Y7xw7pSvcia1alQtVq/8AMqnMw/D+Gn5KLL6E9ZkTE4/EscDoD2e/vP8AOfOPd1tn0fUTpHpjMffspt1Pf5Tq8ocsnEsKlQWoKf8A2EfKPy959B1t5clcvnFVDUqLbD09wd6j9E8FA1PmB3iWqigAAAADQAaAAbDwmnxeIvmoy+Vyn8sn6BERNMzRERAEREATR489sNXPdSqH/A03pr8Ro56VRPxI6/VSIBRqjtEdyr+rf0E9Z5UtTfvVf1b+s9Z8+zfRKfZzgg+Kaof+ilx/aqXQH+6Kg/ilmSE+zCh+7rv+J1X0RM3/ANhk2mzxp1iRj8mt5WIiJOQGUREAxiIgCIiAeeJrqiM7myqCzHuAFyZWeIxD1atSs4sXtYfhQfCnpcg95uessHj1BHw9VXbKuUnNvlK9oG3WxA067SuaL3GosfmXuPd4+fUTN/UapSkvBp/psy6bfkwxKFha9l+Y9bd01OHcOOJrLRoiw+Z7Xso+Jz/IdSR6eXEsbe6rew3tqT4Adf5yzOUuBjC0QGH717God7G2iA/hW5HibnrK3E47yPb8IsczkKFpeTq4DBpRprTpiyoLAfzPeSdSe8z3iJtmIIiIAiIgCIiAJ54j4W8j+hnpBgFCUDsPyr/P+k9pI+a+AU6VfLhyDdQzUswugJIW35TZrA22Nj0HDpYKqzhFpOWY2ACnU2J322B69Jh5MNTXp0bcZZqfUWXyBQy4JD1dqjfVyB/hCyRzQ4BhGpYajTYWZKaK39oKM33vN+bULUpGNb3TYiInRyZREQDGIiAIiRnn7mNsFhw1MA1qhy08wJUWF2Y23sPuRDegcjiPN61cY+Eui0qZtc3vUrI2qg3sFB6EXJTu0mvieCtVcU0OVn0DDoo1a46qB+tgQTKoq1DcsTqxJYN8zMbk37ySSd+ss/2TcDdguNq1C4AqJRXOzAdorVbU2UXWwUd3laq4+JXfgsRl+HOkdvgvIgpVlq1KwqBDmVRTydofCSS7XsbHQDUCTKIk8RMLUoiu6t7piIidnAiIgCIiAIiIAiIgHz97UOJVaPF8Q1GoykrRBta2lJSFINww1vqPmMch8fxuK4hhaRqaLULsQoQlUpuWBIGoIuLeM5nN2GWrj8XUzP2q9QaWPwsUG19LKPS06fsxT3fEqAU2uzgkqbkNSc2ub9dNLbSDp0SbaWj6BiIk5GIiIBlERAMYiIAlH+0rjy4rE2p9qnRBRCD8TE9twb7XAAI3C31BEnntP5k/ZsP7lDatXBAN7ZKd7O1+hI7I63JPQymshAvew8tP1kWSvY6lHnRo1XdUB1JAAGvaY5VFyO+fR/BOGrhqFOgoFqagG3Vt3b1YsfWUhyFhPeY/Dq1iM+e/f7tS4FvNV+kv2e417ihERJDkREQBERAEREAREQBNDmDGmjhq9Yb06bsPMKSPvab853MXD2xGFrUFIU1abICb2BItrbW3lAPnZG2Bve3Xw8esyNwbi9xY6aHQ3BBGoIOokjr+z3iqtpRDgBu0r0yCN9LsrXNh0kYrOwzKVIZTZhsVOlwQe0rWOxGkrNNEmy0uUfaVSye7xrnOtstVUZ84/MEByt42sfPewsBjqVZBUpVFqIdmU3FxuPAjqOko7lrhNF6P7RUIYKxBpr2sttSzAb7N2TbMNrkgSTcqcwFMbSoU1XJWYqyKRpZGbP2RYEBBp+G/p3OTvR68elstOIiTERlERAMYiIBT3MHMarxHEs1PMQUpKdLpTpg5gLnQszMbjoRvPGvzp2WKp2+0VBJAzMMqai+irv3k6S1+I8Ewtc3rUKdQ7XZQW+u85n/AnDN/2Zf71S30zSGsO3slnK0tIjPIS0cRjDXRAvuKNiQuXM9ViAT3sqIw0v8AHudLWTNPhnC6GHUpQpLTUm5Ci1za1z3mwG83JJM+laOKrb2IiJ0ciIiAIiIAiIgCIiAIiIAnK4vy5g8Tf3+HRyRbNaz2/trZh9Z1YgEFr+y/BlrpWr0wd1DKwI7rspNvCd7l7lPCYM5qSEuRY1HOZ7dQOijbYDadyJ56Ue7ERE9PDKIiAIiIAiIgCIiAIiIAiIgCIiAIiIAiIgCIiAIiIAiIgCIiAIiIB//Z",
        }
        ol { class: "",
            li { "This is a list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
            li { "This is another list item" }
        }
    }
}
