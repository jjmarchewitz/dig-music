import Title from "./FilterMenu/Title.jsx"
import "./styles.css";

function SideBar() {
    return (
        <div>
            <div className="fixed top-0 left-0 h-screen w-1/4 m-0
                            text-black bg-gray-900">
                <Title></Title>
                <i>B</i>
                <i>C</i>
                <i>D</i>
                <i>E</i>
            </div>
        </div>
    );
}

function SideBarIcon({ icon }) {

}

export default SideBar;