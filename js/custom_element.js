export function create_element(
    name,
    component
) {
    class CustomElement extends HTMLElement {
        constructor() {
            super();
            component.init(this);
        }

        static get observedAttributes() {
            return component.observed_attributes();
        }

        setData(data = []) {
            component.set_data(data);
        }

        attributeChangedCallback(name, oldValue, newValue) {
            component.attribute_changed_callback(name, oldValue ?? undefined, newValue);
        }

        connectedCallback() {
            component.connected_callback();
        }

        disconnectedCallback() {
            component.disconnected_callback();
        }

        adoptedCallback() {
            component.adopted_callback();
        }
    };
    customElements.define(name, CustomElement);
}
